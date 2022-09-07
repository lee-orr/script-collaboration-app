import LoadingOrError from 'components/LoadingOrError'
import type { ReactElement } from 'react'
import { lazy, Suspense } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { createInMemoryFileList, FileType } from 'utils/FileList'
import { LocalStorageProjectList } from 'utils/LocalStorageProjectList'
import { greet } from 'fountain/fountain'

const MenuPage = lazy(async () => import('pages/Menu'))
const Join = lazy(async () => import('pages/Join'))
const Host = lazy(async () => import('pages/Host'))
const Session = lazy(async () => import('pages/Session'))

const testFileList = createInMemoryFileList([
	{ name: 'test', key: 'test', type: FileType.Fountain }
])

export default function App(): ReactElement {
	greet()
	return (
		<BrowserRouter>
			<Suspense fallback={<LoadingOrError />}>
				<Routes>
					<Route path='/' element={<MenuPage />} />
					<Route path='/join' element={<Join />} />
					<Route path='/join/:presetCode' element={<Join />} />
					<Route
						path='/host'
						element={<Host projects={LocalStorageProjectList} />}
					/>
					<Route
						path='/host/:project/:name'
						element={<Session isHost files={testFileList} />}
					/>
					<Route
						path='/session/:code/:name'
						element={<Session isHost={false} files={testFileList} />}
					/>
				</Routes>
			</Suspense>
		</BrowserRouter>
	)
}
