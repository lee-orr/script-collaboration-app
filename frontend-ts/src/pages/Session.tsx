import { ReactElement } from "react";
import { useParams } from "react-router-dom";

export default function SessionPage({isHost}: {isHost: boolean}): ReactElement {
    const { name, project, code } = useParams<{ name: string | undefined, project: string | undefined, code: string | undefined }>()
    return (
        <div>{isHost ? 'Hosting' : 'Joining'}, {name} @ { project || code || "No code or project"}</div>
    )
}