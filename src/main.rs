use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client};
use std::default::Default;

#[tokio::main]
async fn main() {
    get_instances().await;
}

async fn get_instances() {
    let client = Ec2Client::new(Region::UsEast2);
    let ec2_instances_input: DescribeInstancesRequest = Default::default();
    // ec2_instances_input.max_results = Some(5);
    let instances = client.describe_instances(ec2_instances_input).await;

    match instances {
        Ok(output) => {
            for reservation in output.reservations.iter() {
                for reservations in reservation.iter() {
                    for instances in reservations.instances.iter() {
                        for instance in instances.iter() {
                            println!("{:?}", instance.instance_id)
                        }
                    }
                }
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
