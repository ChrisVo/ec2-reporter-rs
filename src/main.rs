use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client};
use rusoto_sts::{GetCallerIdentityRequest, Sts, StsClient};
use std::default::Default;

#[tokio::main]
async fn main() {
    let account_id = get_account_id().await;
    println!("\n🚀 Getting instances in AWS Account {}\n", account_id);
    let regions = get_regions();
    for region in regions {
        get_instances(&account_id, region).await;
    }
    println!("\n✅ All done!\n")
}

struct RegionInfo<'a> {
    region: Region,
    friendly_name: &'a str,
}

fn get_regions() -> [RegionInfo<'static>; 4] {
    [
        RegionInfo {
            region: Region::UsEast1,
            friendly_name: Region::UsEast1.name(),
        },
        RegionInfo {
            region: Region::UsEast2,
            friendly_name: Region::UsEast2.name(),
        },
        RegionInfo {
            region: Region::UsWest1,
            friendly_name: Region::UsWest1.name(),
        },
        RegionInfo {
            region: Region::UsWest2,
            friendly_name: Region::UsWest2.name(),
        },
    ]
}

async fn get_instances(account_id: &String, region: RegionInfo<'_>) {
    let ec2_client = Ec2Client::new(region.region);
    let ec2_instances_input: DescribeInstancesRequest = Default::default();
    let instances = ec2_client.describe_instances(ec2_instances_input).await;
    println!("\nChecking region {}", region.friendly_name);
    match instances {
        Ok(output) => {
            if output.reservations.as_ref().unwrap().is_empty() {
                println!("{} does not have any instances\n", region.friendly_name);
            } else {
            for reservation in output.reservations.iter() {
                for reservations in reservation.iter() {
                    for instances in reservations.instances.iter() {
                        for instance in instances.iter() {
                            println!("\t- {}", instance.instance_id.as_ref().unwrap())
                        }
                    }
                }
            }
        }
        Err(_) => {
            println!("Couldn't list instances in AWS account {}", account_id)
        }
    }
    println!("\n")
}

async fn get_account_id() -> String {
    let sts_client = StsClient::new(Region::UsEast2);
    let req: GetCallerIdentityRequest = Default::default();
    let caller_identity = sts_client.get_caller_identity(req).await;
    match caller_identity {
        Ok(identity) => {
            identity.account.expect("No account ID detected")
        }
        Err(_) => String::from("Couldn't get account ID")
    }

}