import boto3

ec2 = boto3.client('ec2')
sts = boto3.client('sts')
account_id = sts.get_caller_identity()
instances = ec2.describe_instances()
print(f"\nInstances in AWS Account ID {account_id['Account']}")
for reservation in instances['Reservations']:
    for instance in reservation['Instances']:
        print(f"\t- {instance['InstanceId']}")
print("\n")