import Button from 'components/Button'
import Input from 'components/Input'
import type { ChangeEvent, ReactElement } from 'react'
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
	const [projectKey, setProjectKey] = useState('')

	const onButtonClicked = async (): Promise<void> => {
		let currentProject = projectKey
		if (projectKey === '') {
			currentProject = await projects.createNewProject(project)
		}
		nav(`/host/${currentProject}/${name}`)
	}
	const onSelected = (event: ChangeEvent<HTMLSelectElement>): void => {
		setProjectKey(event.currentTarget.value)
	}
	return (
		<div className='flex h-screen flex-col items-center justify-center gap-4'>
			<h1 className='text-4xl font-bold'>Host Session</h1>
			<span className=''>Your Display Name:</span>
			<Input value={name} input={setName} />
			<span className=''>Choose a project:</span>
			<select
				data-testid='project-selector'
				className='min-w-[200px] border-b-2 border-b-slate-400 bg-transparent p-1'
				value={projectKey}
				onChange={onSelected}
			>
				<option className='text-black' key='' value=''>
					Select a Project...
				</option>
				{projectList.map(p => (
					<option className='text-black' key={p.key} value={p.key}>
						{p.name}
					</option>
				))}
			</select>
			{projectKey === '' ? (
				<>
					<span className=''>Or Create a new one:</span>
					<Input value={project} input={setProject} />
				</>
			) : (
				''
			)}
			<Button
				disabled={name === '' || (projectKey === '' && project === '')}
				click={(): void => {
					void onButtonClicked()
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
