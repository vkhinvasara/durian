use pyo3::prelude::*;
use serde_json::json;

#[pyclass(eq)]
#[derive(PartialEq, Clone)]
enum HTTPMETHOD {
    GET,
    POST, 
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT    
}

#[pyclass]
#[derive(PartialEq, Clone)]
struct Sapota {
    url: String,
    headers: Vec<(String, String)>,
    method: HTTPMETHOD,
    body: Option<String>,
}

fn construct_command(d: &Sapota) -> Result<String, &'static str> {
    let method = match d.method {
        HTTPMETHOD::GET => "GET",
        HTTPMETHOD::POST => "POST",
        HTTPMETHOD::PUT => "PUT",
        HTTPMETHOD::DELETE => "DELETE",
        HTTPMETHOD::HEAD => "HEAD",
        HTTPMETHOD::OPTIONS => "OPTIONS",
        HTTPMETHOD::PATCH => "PATCH",
        HTTPMETHOD::TRACE => "TRACE",
        HTTPMETHOD::CONNECT => "CONNECT",
    };

    let mut command = format!("curl -X {} {}", method, d.url);

    for (key, value) in &d.headers {
        command.push_str(&format!(" -H \"{}: {}\"", key, value));
    }

    if let Some(body) = &d.body {
        command.push_str(&format!(" -d '{}'", body));
    }

    Ok(command)
}

#[pymethods]
impl Sapota {
    #[new]
    fn new(url: String, headers: Vec<(String, String)>, method: HTTPMETHOD, body: Option<String>) -> PyResult<Self> {
       Ok(Sapota { url, headers, method, body })
    }

    fn get_request_command(&self) -> PyResult<String> {
        Ok(construct_command(self).unwrap_or_else(|_| "Error constructing command".to_string()))
    }
}


#[pyclass]
#[derive(PartialEq, Clone)]
struct SapotaCollection{
    collection: Option<Vec<Sapota>>
}

#[pymethods]
impl SapotaCollection{
    #[new]
    fn new(collection: Option<Vec<Sapota>>) -> PyResult<Self>{
        Ok(SapotaCollection { collection })
    }

    fn add_request(&mut self, sapota: &Sapota){
        if let Some(ref mut vec) = self.collection{
            vec.push(sapota.clone());
        } else{
            self.collection = Some(vec![sapota.clone()])
        }
    }

    fn get_collection(&self) -> PyResult<Option<Vec<Sapota>>>{
        Ok(self.collection.clone())
    }
    
    fn export_collection(&self) -> PyResult<String>{
        let mut items = vec![];

        if let Some(ref collection) = self.collection {
            for sapota in collection {
                let method = match sapota.method {
                    HTTPMETHOD::GET => "GET",
                    HTTPMETHOD::POST => "POST",
                    HTTPMETHOD::PUT => "PUT",
                    HTTPMETHOD::DELETE => "DELETE",
                    HTTPMETHOD::HEAD => "HEAD",
                    HTTPMETHOD::OPTIONS => "OPTIONS",
                    HTTPMETHOD::PATCH => "PATCH",
                    HTTPMETHOD::TRACE => "TRACE",
                    HTTPMETHOD::CONNECT => "CONNECT",
                };

                let headers: Vec<_> = sapota.headers.iter().map(|(key, value)| {
                    json!({
                        "key": key,
                        "value": value
                    })
                }).collect();

                let request = json!({
                    "method": method,
                    "header": headers,
                    "body": {
                        "mode": "raw",
                        "raw": sapota.body.clone().unwrap_or_default()
                    },
                    "url": {
                        "raw": sapota.url,
                        "protocol": "https",
                        "host": sapota.url.split('/').collect::<Vec<&str>>()[2],
                        "path": sapota.url.split('/').skip(3).collect::<Vec<&str>>()
                    }
                });

                items.push(json!({
                    "name": sapota.url,
                    "request": request
                }));
            }
        }

        let exported_collection = json!({
            "info": {
                "name": "Sapota Collection",
                "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            },
            "item": items
        });

        Ok(exported_collection.to_string())
    
    }
}


#[pymodule]
fn sapota(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Sapota>()?;
    m.add_class::<SapotaCollection>()?;
    m.add_class::<HTTPMETHOD>()?;
    Ok(())
}