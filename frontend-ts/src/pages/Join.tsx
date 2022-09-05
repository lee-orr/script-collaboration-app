import Button from 'components/Button'
import Input from 'components/Input'
import type { ReactElement} from 'react';
import { useState } from 'react'
import { useNavigate } from 'react-router-dom'

const BACK_INDEX = -1;

export default function JoinPage(): ReactElement {
	const nav = useNavigate()
	const [name, setName] = useState('')
	const [code, setCode] = useState('')
	return (
		<div className='flex h-screen flex-col items-center justify-center gap-4'>
				<h1 className='text-4xl font-bold'>Join Session</h1>
				<span className=''>Session Code:</span>
				<Input value={code} input={setCode} />
				<span className=''>Your Display Name:</span>
				<Input value={name} input={setName} />
				<Button
					click={(): void => {
						nav(`/join/${code}/${name}`)
					}}
					label='Join'
				/>
				<Button
					click={(): void => {
						nav(BACK_INDEX)
					}}
					label='Back'
				/>
			</div>
	)
}
