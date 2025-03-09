import { PostOptions } from "../types";

class MPost {
    declare readonly id: number;
    declare readonly title: string;
    declare readonly description: string;

    constructor(options: PostOptions) {
        Object.assign(this, options);
    };
    
}

export { MPost }