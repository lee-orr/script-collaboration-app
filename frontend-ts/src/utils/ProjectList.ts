export interface Project {
	name: string
	key: string
}

export interface ProjectList {
	getProjectList: () => Project[]
	createNewProject: (name: string) => Promise<string>
	deleteProject: (id: string) => Promise<void>
}

export function createInMemoryProjectList(list: Project[]) : ProjectList & {list: Project[]} {
	return {
		list,
		getProjectList() {
			return list;
		},
		async createNewProject(name) {
			let key = name.toLowerCase().replaceAll(/\s/g, '-');
			this.list = [
			...(this.list.filter((p) => p.key !== key) || []),
			{ name, key }
		];
	return key
},
		async deleteProject(id) {this.list = this.list
			? this.list.filter(p => p.key !== id)
			: []}}
}