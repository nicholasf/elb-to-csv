Downloads Classic ELB logs from an S3 bucket and stores them in a postgres database.

ELB Logging on AWS is explained at http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/access-log-collection.html .

## Log syntax.

```
timestamp elb client:port backend:port request_processing_time backend_processing_time response_processing_time elb_status_code backend_status_code received_bytes sent_bytes "request" "user_agent" ssl_cipher ssl_protocol
```

This program was originally written to generate CSV data for JMeter load tests. It can be still be used for this purpose.


elb-logs-to-postgres.