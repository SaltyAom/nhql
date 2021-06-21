import * as gcp from '@pulumi/gcp'

import { projectName, imageName } from '../config'

// ? Google Cloud Run Deployment
const location = gcp.config.region || 'asia-east1'

const service = new gcp.cloudrun.Service(
	projectName,
	{
		location,
		// metadata: {
		// 	namespace: projectName
		// },
		template: {
			spec: {
				containerConcurrency: 1000,
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
	}
)

// const defaultDomainMapping = new gcp.cloudrun.DomainMapping(
// 	'defaultDomainMapping',
// 	{
// 		location,
// 		metadata: {
// 			namespace: projectName
// 		},
// 		spec: {
// 			routeName: service.name
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
