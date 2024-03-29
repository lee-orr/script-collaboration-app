import Button from 'components/Button'
import Input from 'components/Input'
import type { ReactElement } from 'react'
import { useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'

export default function JoinPage(): ReactElement {
	const nav = useNavigate()
	const { presetCode } = useParams<{ presetCode: string | undefined }>()
	const [name, setName] = useState('')
	const [code, setCode] = useState(presetCode ?? '')
	return (
		<div className='flex h-screen flex-col items-center justify-center gap-4'>
			<h1 className='text-4xl font-bold'>Join Session</h1>
			<span className=''>Session Code:</span>
			<Input value={code} input={setCode} />
			<span className=''>Your Display Name:</span>
			<Input value={name} input={setName} />
			<Button
				disabled={name === '' || code === ''}
				click={(): void => {
					nav(`/session/${code}/${name}`)
				}}
				label='Join'
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
