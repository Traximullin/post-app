import { Post } from "@components/post";
import { MPost } from "@models/post";

function VWall() {
    const posts = [
        new MPost({ id: 1, title: "Первый пост", description: "Описание первого поста" }),
        new MPost({ id: 2, title: "Второй пост", description: "Описание второго поста" }),
        new MPost({ id: 3, title: "Третий пост", description: "Описание третьего поста" }),
        new MPost({ id: 4, title: "Четвертый пост", description: "Описание четвертого поста" })
    ];

    return (
        <div className="w-full max-w-md p-4 space-y-4">
            {posts.map(({id, title, description}) => (
                <Post 
                    key={id} 
                    title={title}  
                    description={description} 
                />
            ))}
        </div>
    );
}

export { VWall }