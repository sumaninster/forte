pub mod api {
    use reqwest::Client;
    use std::io::{ErrorKind, Error};
    use json::JsonValue;
    use serde_json::json;
    use serde::{Deserialize, Serialize};
    use snake_case::SnakeCase;
    use std::env;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Data {
        id: String,
        data: String,
    }

    fn auth() -> Result<(String, String), Error> {
        let user = match env::var("USER"){
            Ok(u) => u,
            Err(e) => return Err(Error::new(ErrorKind::Other, "Username not found!")),
        };
        let pass = match env::var("PASS"){
            Ok(u) => u,
            Err(e) => return Err(Error::new(ErrorKind::Other, "Password not found!")),
        };
        Ok((user, pass))
    }

    pub async fn call_api(latitude: i64, longitude: i64, status: &String, request_url: &String) -> Result<Data, Error> {
        if status.ne("AtDestination") ||  status.ne("BeingPrepared") || status.ne("Delayed") || status.ne("InTransit") {
            return Err(Error::new(ErrorKind::Other, "Unsupported Status!"));
        }
        let status = SnakeCase::try_from_str(status.as_str());
        println!("{} : {}", request_url, status.unwrap().to_string());
        let mut request_data = json::JsonValue::new_object();
        let mut position = json::JsonValue::new_object();
        position["latitude"] = JsonValue::from(latitude);
        position["longitude"] = JsonValue::from(longitude);
        request_data["position"] = position;
        request_data["status"] = JsonValue::from(status.unwrap().to_string());
        let body = json!(&json::stringify(request_data));
        let (user, pass) = auth()?;
        let response = Client::new()
            .post(request_url)
            .basic_auth(user, Some(pass))
            .json(&body)
            .send().await;
        let response = match response {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                return Err(Error::new(ErrorKind::Other, "Response Error!"))
            },
        };
        let result = response.json().await;
        let result = match result {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                return Err(Error::new(ErrorKind::Other, "Result Error!"))
            },
        };
        Ok(result)
   }

    pub async fn try_call_api(latitude: i64, longitude: i64, status: &String, request_url: &String, retry: i32) {
        let result = call_api(45, 120, status, request_url).await;
        match result {
            Ok(response) => { println!("{:?}", response) },
            Err(e) => {
                println!("{:?}", e);
                if retry > 0 {
                    try_call_api(latitude, longitude, status, request_url, retry - 1);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{try_call_api};

    #[test]
    fn it_works() {
        let id = 1;
        let request_url = format!("http://localhost/api/{}", id);
        try_call_api(45, 120, &"AtDestination".to_string(), &request_url, 5).await;
    }
}
