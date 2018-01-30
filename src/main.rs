
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate chrono;

use chrono::*;
use std::default::Default;

use std::io::prelude::*;
use std::fs::File;

use std::fs::OpenOptions;

use rusoto_core::{ default_tls_client, ProfileProvider, Region };
use rusoto_s3::{ S3, S3Client, ListObjectsRequest, GetObjectRequest };

fn main() {
    println!("Beginning.");
    let mut provider = ProfileProvider::new().unwrap();
    provider.set_file_path("/Users/nicholasf/.aws/credentials.properties");
    let client = S3Client::new(default_tls_client().unwrap(), provider, Region::ApSoutheast2);
    // let bucket = "s3://ops-prod-logging-1/tcog/prod/AWSLogs/877800914193/elasticloadbalancing/ap-southeast-2/2016/12/18".to_string();
    let bucket = "ops-prod-logging-1".to_string();
    let prefix = Some("tcog/tcog-nca/AWSLogs/877800914193/elasticloadbalancing/ap-southeast-2/2017/06/26".to_string());
    let delimiter = Some("|".to_string());

    let list_objects_request: ListObjectsRequest = ListObjectsRequest {
        bucket: bucket,
        prefix: prefix,
        max_keys: Some(500),
        delimiter: delimiter,
        marker: None,
        encoding_type: None,
        request_payer: None,
    };

    println!("Ok, requesting the list of objects.");

    let mut object_request: GetObjectRequest = Default::default();
    object_request.bucket = "ops-prod-logging-1".to_string();

    match client.list_objects(&list_objects_request) {
        Ok(output) => {
            for content in output.contents.unwrap() {
                println!("{:?}", content.key.unwrap());
                object_request.key = content.key.unwrap();

                match client.get_object(&object_request) {
                    Ok(output) => {
                        let streamingBody = output.body.unwrap();
                        let log = String::from_utf8(streamingBody);
                        let csv = generate_csv(log);
                        match OpenOptions::new().write(true).open("out.csv") {
                            Ok(mut f) => {
                                for line in csv {
                                    match f.write_all(line.as_bytes()) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!("Error: {:?}", err);
                                        }
                                    } 
                                }
                            }
                            Err(err) => {
                                println!("Error: {:?}", err);
                            }                            
                        }
                    }
                    Err(error) => {
                        println!("Error: {:?}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

//fn get_object_list(bucket &str, max_keys i32) => Vec s3:Object]{}
//    let mut csv = Vec::new();

fn generate_csv(log: String) -> Vec<String> {
    let mut log_lines: Vec<&str> = log.split("- -").collect();
    let mut csv: Vec<String> = Vec::new();

    for line in log_lines {
        match parse_log_into_csv(line.to_string()) {
            Ok(csv_line) => {
                println!("{}", csv_line);
                csv.push(csv_line);
            }
            Err(_) => {
                println!("Malformed!");
            }
        }
    }

    return csv;
}

fn parse_log_into_csv(line: String) -> Result<String, &'static str> {
    let mut v: Vec<&str> = line.split(" ").collect();

    if v.len() < 14 || v[13].len() == 0 {
        println!(">> {}", line);
        return Err("Malformed.");
    }

    // let epoch_timespec = time::get_time(v[1]);
    // let mut t = epoch_timespec.sec.to_string().to_owned();
    // let nanosecs = epoch_timespec.nsec.to_string();
    // t.push(nanosecs);

    match v[0].parse::<DateTime<UTC>>() {
        Ok(t) => {
            let mut entry = "".to_string();
            // let mut entry = t.timestamp().to_string();
            // let mut millis = t.timestamp_subsec_millis().to_string();
            // entry.push_str(millis.as_str());

            // entry.push_str(" \t ");

            let uri = v[12].replace("http://nca.tcog.news.com.au:80", "");
            entry.push_str(uri.as_str());
            entry.push_str("\n");
            println!("{}", entry);
            return Ok(entry);
        }        
        Err(err) => {
            println!("err!");
            println!("{:?}", err);
            return Err("Malformed");
        }
    }
}
