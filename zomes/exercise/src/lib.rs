use hdk::prelude::*;

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePostInput {
    content: String,
}

// create an entry from input data. create a link from user's public key to new post
#[hdk_extern]
pub fn create_post(external_data: CreatePostInput) -> ExternResult<EntryHash> {
    let post: Post = Post(external_data.content); 
    create_entry(&post)?;
    let post_hash = hash_entry(&post)?;

    // into() changes HoloHash<Agent> into HoloHash<Entry> for linking
    let user_pubkey = agent_info()?.agent_initial_pubkey.into();

    create_link(
        user_pubkey, 
        post_hash.clone(), 
        LinkTag::new(String::from("author"))
    )?;
    Ok(post_hash)
}

// given an agent's public key, retrieve all posts created by them
#[hdk_extern]
pub fn get_posts_for_agent(agent_pubkey: AgentPubKey) -> ExternResult<Vec<Post>> {
    let agent_links = get_links(agent_pubkey.into(), None)?;
    let mut agent_posts = vec![];

    for link in agent_links {
        // get the Element representation of the post
        let element: Element = get(link.target, GetOptions::default())?
            .ok_or(WasmError::Guest(String::from("Entry not found")))?;

        // deserialise the Element
        let entry_option: Option<Post> = element.entry().to_app_option()?;

        // extract value as Post type from Some, return error if None
        let entry: Post = entry_option
            .ok_or(WasmError::Guest("The targeted entry is not a post".into()))?;

        // add this to the vector        
        agent_posts.push(entry);
    }
    Ok(agent_posts)
}
