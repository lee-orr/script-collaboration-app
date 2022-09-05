import Button from 'components/Button'
import Input from 'components/Input'
import type { ReactElement } from 'react'
import { useState } from 'react'
import { useNavigate } from 'react-router-dom'

export default function HostPage(): ReactElement {
	const nav = useNavigate()
	const [name, setName] = useState('')
	return (
		<div className='flex h-screen flex-col items-center justify-center gap-4'>
			<h1 className='text-4xl font-bold'>Host Session</h1>
			<span className=''>Your Display Name:</span>
			<Input value={name} input={setName} />
			<Button
				disabled={name === ''}
				click={(): void => {}}
				label='Choose The Hosted Folder'
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
