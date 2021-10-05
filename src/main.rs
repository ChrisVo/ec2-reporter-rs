use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client};
use std::default::Default;

#[tokio::main]
async fn main() {
    let client = Ec2Client::new(Region::UsEast2);
    let ec2_instances_input: DescribeInstancesRequest = Default::default();
    
    match client.describe_instances(ec2_instances_input).await {
        Ok(output) => println!("{:?}", output),
        Err(error) => println!("{:?}", error)
    }
}
