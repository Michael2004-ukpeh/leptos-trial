use leptos::*;


#[component]
pub fn Navbar() -> impl IntoView{
    view! {
        <nav class="flex mx-[3rem] my-0  justify-around items-center border-b-2 border-[#000000] py-[1.5rem]">
            <h1 class="text-wrap text-2xl">Ukpeh Michael</h1>
            <ul class="list-none flex justify-around gap-3">
                <li>
                    <a href="#">Home</a>
                </li>
                <li>
                    <a href="#">About</a>
                </li>
                <li>
                    <a href="#">Works</a>
                </li>

            </ul>

        </nav>
    }
}