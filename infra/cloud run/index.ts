import * as gcp from '@pulumi/gcp'

import { projectName, imageName } from '../config'

// ? Google Cloud Run Deployment
const location = gcp.config.region || 'asia-east1'

// const enableCloudRun = new gcp.projects.Service('EnableCloudRun', {
// 	service: 'run.googleapis.com'
// })

const service = new gcp.cloudrun.Service(
	projectName,
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
	// { dependsOn: enableCloudRun }
)

// const defaultDomainMapping = new gcp.cloudrun.DomainMapping(
// 	'defaultDomainMapping',
// 	{
// 		location,
// 		metadata: {
// 			namespace: 'nhql'
// 		},
// 		spec: {
// 			routeName: 'nhql'
// 		}
// 	}
// )

const iam = new gcp.cloudrun.IamMember(`${projectName}-iam`, {
	service: service.name,
	location,
	role: 'roles/run.invoker',
	member: 'allUsers'
})

export const revision = service.statuses[0].latestCreatedRevisionName
export const url = service.statuses[0].url
