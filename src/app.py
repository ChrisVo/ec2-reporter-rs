import boto3

ec2 = boto3.client('ec2')
instances = ec2.describe_instances()
for reservation in instances['Reservations']:
    for instance in reservation['Instances']:
        print(instance['InstanceId'])