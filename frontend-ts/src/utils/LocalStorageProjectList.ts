import type { Project, ProjectList } from './ProjectList'

const item = localStorage.getItem('projects')
const projectList: Project[] = item ? JSON.parse(item) : []

export const LocalStorageProjectList: ProjectList & {
	projectList: Project[]
} = {
	projectList,
	getProjectList(): Project[] {
        const item = localStorage.getItem('projects')
        const projectList: Project[] = item ? JSON.parse(item) : []
		return this.projectList
	},
	async createNewProject(name: string): Promise<string> {
		let key = name.toLowerCase().replaceAll(/\s/g, '-')
		this.projectList = [
			...(this.projectList.filter((p) => p.key !== key) || []),
			{ name, key }
		]
		localStorage.setItem('projects', JSON.stringify(this.projectList))
		return key
	},
	async deleteProject(id: string): Promise<void> {
		this.projectList = this.projectList
			? this.projectList.filter(p => p.key !== id)
			: []
		localStorage.setItem('projects', JSON.stringify(this.projectList))
	}
}

export default LocalStorageProjectList
