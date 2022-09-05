import Button from 'components/Button'
import Input from 'components/Input'
import type { ReactElement } from 'react'
import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import type { ProjectList } from 'utils/ProjectList'

export default function HostPage({
	projects
}: {
	projects: ProjectList
}): ReactElement {
	const nav = useNavigate()
	const [name, setName] = useState('')
	const [projectList] = useState(projects.getProjectList())
	const [project, setProject] = useState('')
	return (
		<div className='flex h-screen flex-col items-center justify-center gap-4'>
			<h1 className='text-4xl font-bold'>Host Session</h1>
			<span className=''>Your Display Name:</span>
			<Input value={name} input={setName} />
			<span className=''>Choose a project:</span>
			<select data-testid={'project-selector'} className='border-b-2 border-b-slate-400 bg-transparent p-1 min-w-[200px]' value={project} onChange={(event) => setProject(event.currentTarget.value)}>
				<option className="text-black" key="" value="">Select a Project...</option>
				{projectList.map((project) => (<option className="text-black" key={project.key} value={project.key}>{project.name}</option>))}
			</select>
			<span className=''>Or Create a new one:</span>
			<Input value={project} input={setProject}/>
			<Button
				disabled={name === '' || project === ''}
				click={async (): Promise<void> => {
					let projectExists = projectList.find((p) => p.key === project)
					let currentProject = project;
					if (!projectExists) {
						currentProject = await projects.createNewProject(project)
					}
					nav(`/host/${currentProject}/${name}`)
				}}
				label='Start Session'
			/>
			<Button
				click={(): void => {
					nav('/')
				}}
				label='Back'
			/>
		</div>
	)
}
