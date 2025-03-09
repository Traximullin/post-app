import type { HTMLAttributes } from "react";

interface PostProps extends Omit<HTMLAttributes<HTMLDivElement>, 'className'> {
    title: string;
    description: string;
}

function Post(props: PostProps) {
    const { title, description, ...othersProps } = props;

    return (
        <div 
            className="p-4 border rounded-lg shadow-md bg-white" 
            {...othersProps}
        >
            <h2 className="text-lg font-bold">{title}</h2>
            <p className="text-gray-600">{description}</p>
        </div>
    )
}

export { Post, type PostProps };
