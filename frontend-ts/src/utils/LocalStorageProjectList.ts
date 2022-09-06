import { GenerateKey } from './KeyGenerator'
import type { Project, ProjectList } from './ProjectList'

const item = localStorage.getItem('projects')
const projectList: Project[] = item ? (JSON.parse(item) as Project[]) : []

export const LocalStorageProjectList: ProjectList & {
	projectList: Project[]
} = {
	projectList,
	getProjectList(): Project[] {
		return this.projectList
	},
	async createNewProject(name: string): Promise<string> {
		const key = GenerateKey()
		this.projectList = [
			...this.projectList.filter(p => p.key !== key),
			{ name, key }
		]
		localStorage.setItem('projects', JSON.stringify(this.projectList))
		return key
	},
	async deleteProject(id: string): Promise<void> {
		this.projectList = this.projectList.filter(p => p.key !== id)
		localStorage.setItem('projects', JSON.stringify(this.projectList))
	}
}

export default LocalStorageProjectList
