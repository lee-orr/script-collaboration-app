import { v4 as uuidv4 } from 'uuid'

export function GenerateKey(): string {
	return uuidv4()
}

export default GenerateKey
