use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BookNavbarProps {}

#[function_component(BookNavbarComponent)]
pub fn book_navbar(_props: &BookNavbarProps) -> VNode {
    html! {
        <nav class="my-4 px-5 border-2 border-primary-200 dark:border-gray-400 rounded-2xl">
            <ul class="flex flex-wrap md:flex-row list-none gap-2 place-content-center">
                <li tabindex="0" aria-label="Export Metadata" role="link"><a
                        class="flex flex-col items-center w-16 md:w-32 text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400 dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline"
                        title="Export Metadata" href="#export-metadata"><svg xmlns="http://www.w3.org/2000/svg" fill="none"
                            viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon"
                            class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M7.5 7.5h-.75A2.25 2.25 0 0 0 4.5 9.75v7.5a2.25 2.25 0 0 0 2.25 2.25h7.5a2.25 2.25 0 0 0 2.25-2.25v-7.5a2.25 2.25 0 0 0-2.25-2.25h-.75m-6 3.75 3 3m0 0 3-3m-3 3V1.5m6 9h.75a2.25 2.25 0 0 1 2.25 2.25v7.5a2.25 2.25 0 0 1-2.25 2.25h-7.5a2.25 2.25 0 0 1-2.25-2.25v-.75">
                            </path>
                        </svg><span class="hidden md:block">{"Export Metadata"}</span></a></li>
                <li tabindex="0" aria-label="Metadata" role="link"><a
                        class="flex flex-col items-center w-16 md:w-32 text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400 dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline"
                        title="Metadata" href="#metadata"><svg xmlns="http://www.w3.org/2000/svg" fill="none"
                            viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon"
                            class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M7.864 4.243A7.5 7.5 0 0 1 19.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 0 0 4.5 10.5a7.464 7.464 0 0 1-1.15 3.993m1.989 3.559A11.209 11.209 0 0 0 8.25 10.5a3.75 3.75 0 1 1 7.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 0 1-3.6 9.75m6.633-4.596a18.666 18.666 0 0 1-2.485 5.33">
                            </path>
                        </svg><span class="hidden md:block">{"Metadata"}</span></a>
                </li>
                <li tabindex="0" aria-label="Locations" role="link"><a
                        class="flex flex-col items-center w-16 md:w-32 text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400 dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline"
                        title="Locations" href="#locations"><svg xmlns="http://www.w3.org/2000/svg" fill="none"
                            viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon"
                            class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244">
                            </path>
                        </svg><span class="hidden md:block">{"Locations"}</span></a></li>
                <li tabindex="0" aria-label="Contributors" role="link"><a class="flex flex-col items-center w-16 md:w-32 text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400 dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent
                           hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline"
                        title="Contributors" href="#contributors"><svg xmlns="http://www.w3.org/2000/svg" fill="none"
                            viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon"
                            class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6 18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0 0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Z">
                            </path>
                        </svg><span class="hidden md:block">{"Contributors"}</span></a>
                </li>
            </ul>
        </nav>
    }
}
