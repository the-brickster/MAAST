// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = false;

export async function load({data}){
    return {projects:[
        {dashboardName:"Rust-data-processor",urlPath:"D:\\Documents\\Programming\\Rust_projects\\rust-data-processor"},
        {dashboardName:"Doom",urlPath:"D:\\Documents\\Games\\Doom"},
        {dashboardName:"Cool Game",urlPath:"D:\\Documents\\Games\\CoolGame"},
        {dashboardName:"Chex Quest",urlPath:"D:\\Documents\\Dashboards\\ChexQuest"},
    ]};
}