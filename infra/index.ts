import * as docker from '@pulumi/docker'
import * as gcp from '@pulumi/gcp'
import * as pulumi from '@pulumi/pulumi'

// ? Build and push image to gcr repository
const projectName = 'nhql'

const imageName = pulumi.interpolate`asia.gcr.io/${gcp.config.project}/${projectName}:latest`

const myImage = new docker.Image(projectName, {
	imageName,
	build: {
		context: '../code'
	}
})

// Digest exported so it's easy to match updates happening in cloud run project
export const digest = myImage.digest

// ? Google Cloud Run Deployment
const location = gcp.config.region || 'asia-east1'

// ? Uncomment if you're using dedicated project for Cloud Run
// const enableCloudRun = new gcp.projects.Service('EnableCloudRun', {
// 	service: 'run.googleapis.com'
// })

const service = new gcp.cloudrun.Service(
	'nhql',
	{
		location,
		template: {
			spec: {
				containers: [
					{
						image: imageName,
						resources: {
							limits: {
								cpu: '2000m',
								memory: '512Mi'
							}
						}
					}
				]
			}
		}
	},
	// ? Uncomment if you're using dedicated project for Cloud Run
	// { dependsOn: enableCloudRun }
)

const iam = new gcp.cloudrun.IamMember('nhql-iam', {
	service: service.name,
	location,
	role: 'roles/run.invoker',
	member: 'allUsers'
})

export const url = service.statuses[0].url