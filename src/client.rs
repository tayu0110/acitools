mod query;

use crate::{AaaLogin, Response};
pub use query::*;
use reqwest::{
    cookie::Jar,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};
use serde_json::json;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    username: String,
    pub(crate) endpoint: Url,
    domain: Option<String>,
    password: String,
    pub(crate) client: reqwest::Client,
    jar: Arc<Jar>,
}

impl Client {
    pub async fn new(
        username: &str,
        endpoint: &str,
        domain: &str,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let jar = Arc::new(Jar::default());

        let mut header = HeaderMap::new();
        header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&jar))
            .default_headers(header)
            .danger_accept_invalid_certs(true)
            .build()?;
        let endpoint = Url::parse(&format!("https://{}", endpoint))?.join("api/")?;
        let res = Self::login(&client, username, &endpoint, domain, password).await?;

        jar.add_cookie_str(format!("APIC-cookie={}", res.token()).as_str(), &endpoint);

        Ok(Self {
            username: username.to_string(),
            endpoint,
            domain: domain.is_empty().then_some(domain.to_string()),
            password: password.to_string(),
            client,
            jar,
        })
    }

    async fn set_cookie(&self, token: &str) {
        self.jar
            .add_cookie_str(format!("APIC-cookie={}", token).as_str(), &self.endpoint);
    }

    async fn login(
        client: &reqwest::Client,
        username: &str,
        endpoint: &Url,
        domain: &str,
        password: &str,
    ) -> Result<AaaLogin, Box<dyn std::error::Error>> {
        // https://community.cisco.com/t5/application-centric-infrastructure/rest-api-how-to-specify-domain-in-the-login-data/m-p/3006196
        // If we want to specify the Login Domain, we can use the format "apic:domain\\name" for "name".
        let login = json!({
            "aaaUser" : {
                "attributes" : {
                    "name" : if domain.is_empty() { username.to_string() } else { format!("apic:{}\\{}", domain, username) },
                    "pwd" : password
                }
            }
        }).to_string();

        let res = client
            .post(endpoint.join("aaaLogin.json")?)
            .body(login)
            .send()
            .await?
            .json::<Response>()
            .await?
            .extract()
            .await?
            .pop()
            .unwrap();

        Ok(serde_json::from_value::<AaaLogin>(res)?)
    }

    pub async fn refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut login) = self
            .get_unchecked(self.endpoint.join("aaaRefresh.json")?, &[])
            .await
        {
            let token = serde_json::from_value::<AaaLogin>(login.pop().unwrap())?.token();
            self.set_cookie(&token).await;
            return Ok(());
        }
        let token = Self::login(
            &self.client,
            &self.username,
            &self.endpoint,
            &self.domain.as_ref().unwrap_or(&"".to_string()),
            &self.password,
        )
        .await?
        .token();
        self.set_cookie(&token).await;
        Ok(())
    }

    pub(crate) async fn get_unchecked(
        &self,
        endpoint: Url,
        queries: &[(&'static str, String)],
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get(endpoint)
            .query(queries)
            .send()
            .await?
            .json::<Response>()
            .await?
            .extract()
            .await?)
    }

    pub fn get(&self, endpoint: &str) -> Result<GetRequestBuilder, Box<dyn std::error::Error>> {
        Ok(GetRequestBuilder::new(
            self,
            self.endpoint.clone().join(endpoint)?,
        ))
    }

    async fn post_unchecked(
        &self,
        endpoint: &str,
        json: &serde_json::Value,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .post(self.endpoint.join(endpoint)?)
            .body(json.to_string())
            .send()
            .await?
            .json::<Response>()
            .await?
            .extract()
            .await?)
    }

    pub async fn post(
        &self,
        endpoint: &str,
        json: &serde_json::Value,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let res = self.post_unchecked(endpoint, json).await;
        if res.is_ok() {
            return res;
        }
        self.refresh().await?;
        Ok(self.post_unchecked(endpoint, json).await?)
    }
}

pub struct GetRequestBuilder<'a> {
    client: &'a Client,
    endpoint: Url,
    queries: Vec<String>,
}

impl<'a> GetRequestBuilder<'a> {
    fn new(client: &'a Client, endpoint: Url) -> Self {
        Self {
            client,
            endpoint,
            queries: vec![],
        }
    }

    fn join(&mut self, field: impl ToString, query: &str) {
        self.queries
            .push(format!("{}={}", query, field.to_string()));
    }

    pub fn query_target(mut self, target: QueryTarget) -> Self {
        self.join(target, "query-target");
        self
    }

    pub fn target_subtree_class(mut self, class_name: ClassName) -> Self {
        self.join(class_name, "target-subtree-class");
        self
    }

    pub fn query_target_filter(mut self, filter: Filter) -> Self {
        self.join(filter, "query-target-filter");
        self
    }

    pub fn rsp_subtree(mut self, rsp_subtree: RspSubTree) -> Self {
        self.join(rsp_subtree, "rsp-subtree");
        self
    }

    pub fn rsp_subtree_class(mut self, class_name: ClassName) -> Self {
        self.join(class_name, "rsp-subtree-class");
        self
    }

    pub fn rsp_subtree_filter(mut self, filter: Filter) -> Self {
        self.join(filter, "rsp-subtree-filter");
        self
    }

    pub fn rsp_prop_include(mut self, rsp_prop_include: RspPropInclude) -> Self {
        self.join(rsp_prop_include, "rsp-prop-include");
        self
    }

    pub async fn send(mut self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.endpoint
            .set_query(Some(&self.queries.join("&").to_string()));
        let res = self.client.get_unchecked(self.endpoint.clone(), &[]).await;
        if res.is_ok() {
            return res;
        }
        self.client.refresh().await?;
        Ok(self.client.get_unchecked(self.endpoint, &[]).await?)
    }
}

pub trait BuilderTrait<'a>: Sized {
    fn renew(builder: GetRequestBuilder<'a>) -> Self;
    fn builder(self) -> GetRequestBuilder<'a>;
    fn query_target(self, target: QueryTarget) -> Self {
        Self::renew(self.builder().query_target(target))
    }
    fn target_subtree_class(self, class_name: ClassName) -> Self {
        Self::renew(self.builder().target_subtree_class(class_name))
    }
    fn query_target_filter(self, filter: Filter) -> Self {
        Self::renew(self.builder().query_target_filter(filter))
    }
    fn rsp_subtree(self, rsp_subtree: RspSubTree) -> Self {
        Self::renew(self.builder().rsp_subtree(rsp_subtree))
    }
    fn rsp_subtree_class(self, class_name: ClassName) -> Self {
        Self::renew(self.builder().rsp_subtree_class(class_name))
    }
    fn rsp_subtree_filter(self, filter: Filter) -> Self {
        Self::renew(self.builder().rsp_subtree_filter(filter))
    }
    fn rsp_prop_include(self, rsp_prop_include: RspPropInclude) -> Self {
        Self::renew(self.builder().rsp_prop_include(rsp_prop_include))
    }
}
