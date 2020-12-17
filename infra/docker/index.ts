import * as docker from '@pulumi/docker'

import { projectName, imageName } from '../config'

// ? Build and push image to gcr repository
const myImage = new docker.Image(projectName, {
	imageName,
	build: {
		context: '../../code'
	}
})

// Digest exported so it's easy to match updates happening in cloud run project
export const digest = myImage.digest
