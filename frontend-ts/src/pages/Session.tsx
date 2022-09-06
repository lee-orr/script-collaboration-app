import type { ReactElement } from "react";
import { useParams } from "react-router-dom";
import Split from "react-split";

export default function SessionPage({isHost}: {isHost: boolean}): ReactElement {
    const { name, project, code } = useParams<{ name: string | undefined, project: string | undefined, code: string | undefined }>()
    return (
        <div className="h-screen flex flex-col items-stretch justify-start">
            <div className="bg-slate-900 p-2 flex flex-row justify-center">{isHost ? 'Hosting' : 'Joining'}, {name} @ { project ?? code ?? "No code or project"}</div>
            <div className="flex flex-row flex-grow">
                <div className="bg-slate-800 border-r-2 border-r-slate-900 p-2 flex flex-col justify-start w-56">Here</div>
                <Split className="split flex flex-row flex-grow">
                    <div className="p-2">Test</div>
                    <div className="p-2">Me</div>
                </Split>
            </div>
        </div>
    )
}