use uuid::Uuid;
use yew::{html, Context};
use yew::{Component, Html, Properties};
use yewtil::fetch::{Fetch, FetchAction, FetchState};

use crate::models::book::book_query::{
    FetchActionBook, FetchWork, Variables, WorkRequest, WorkRequestBody,
};
use crate::models::utils::{
    ImprintWithPublisher, Orcid, SubjectType, ThothError, WorkStatus, WorkStatusValues, WorkType,
    WorkTypeValues, WorkWithRelations, DOI_DOMAIN, ROR_DOMAIN,
};
use crate::THOTH_EXPORT_API;

use crate::component::utils::Loader;

pub struct BookDetailComponent {
    book: WorkWithRelations,
    // Track the user-entered DOI string, which may not be validly formatted
    doi: String,
    doi_warning: String,
    // Track imprint stored in database, as distinct from imprint selected in dropdown
    imprint_id: Uuid,
    // Track work_type stored in database, as distinct from work_type selected in dropdown
    work_type: WorkType,
    data: WorkFormData,
    fetch_work: FetchWork,
    book_id: Uuid,
}

#[derive(Default)]
struct WorkFormData {
    imprints: Vec<ImprintWithPublisher>,
    work_types: Vec<WorkTypeValues>,
    work_statuses: Vec<WorkStatusValues>,
}

pub enum Msg {
    GetBook,
    SetBookFetchState(FetchActionBook),
}

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: Uuid,
}

impl Component for BookDetailComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let fetch_work: FetchWork = Default::default();
        let book: WorkWithRelations = Default::default();
        let doi = Default::default();
        let doi_warning = Default::default();
        let imprint_id = book.imprint.imprint_id;
        let work_type = book.work_type.clone();
        let data: WorkFormData = Default::default();
        let book_id = ctx.props().book_id;

        ctx.link().send_message(Msg::GetBook);
        BookDetailComponent {
            book,
            doi,
            doi_warning,
            imprint_id,
            work_type,
            data,
            fetch_work,
            book_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetBookFetchState(fetch_state) => {
                self.fetch_work.apply(fetch_state);
                match self.fetch_work.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => {
                        self.book = match &body.data.work {
                            Some(w) => w.to_owned(),
                            None => Default::default(),
                        };
                        // Initialise user-entered DOI variable to match DOI in database
                        self.doi = self.book.doi.clone().unwrap_or_default().to_string();
                        self.imprint_id = self.book.imprint.imprint_id;
                        self.work_type = self.book.work_type.clone();
                        body.data.imprints.clone_into(&mut self.data.imprints);
                        body.data
                            .work_types
                            .enum_values
                            .clone_into(&mut self.data.work_types);
                        body.data
                            .work_statuses
                            .enum_values
                            .clone_into(&mut self.data.work_statuses);
                        true
                    }
                    FetchState::Failed(_, _err) => false,
                }
            }
            Msg::GetBook => {
                let body = WorkRequestBody {
                    variables: Variables {
                        work_id: Some(ctx.props().book_id),
                        publishers: None,
                    },
                    ..Default::default()
                };
                let request = WorkRequest { body };
                self.fetch_work = Fetch::new(request);

                ctx.link()
                    .send_future(self.fetch_work.fetch(Msg::SetBookFetchState));
                ctx.link()
                    .send_message(Msg::SetBookFetchState(FetchAction::Fetching));
                false
            }
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        match self.fetch_work.as_ref().state() {
            FetchState::NotFetching(_) => html! {<Loader/>},
            FetchState::Fetching(_) => html! {<Loader/>},
            FetchState::Fetched(_body) => {
                // let languages = match &self.book.languages {
                //     Some(w) => w
                //         .to_owned()
                //         .get(0)
                //         .map_or_else(Default::default, |l| l.language_relation.clone()),
                //     None => Default::default(),
                // };
                let contributors_text = match self.book.contributions.clone() {
                    Some(w) => {
                        let c = w
                            .clone()
                            .iter()
                            .map(|c| {
                                format!("{}({})", c.full_name.clone(), c.contribution_type.clone())
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        c
                    }
                    None => Default::default(),
                };
                let lccn = self.book.lccn.clone().unwrap_or_default().to_string();
                let landing_page = self
                    .book
                    .landing_page
                    .clone()
                    .unwrap_or_default()
                    .to_string();
                let license = self.book.license.clone().unwrap_or_default().to_string();
                let copyright_holder = self
                    .book
                    .copyright_holder
                    .clone()
                    .unwrap_or_default()
                    .to_string();
                let place_of_publication = self.book.place.clone().unwrap_or_default().to_string();
                let publication_date = self
                    .book
                    .publication_date
                    .clone()
                    .unwrap_or_default()
                    .to_string();
                let publications = match self.book.publications.clone() {
                    Some(p) => p.to_owned(),
                    None => Default::default(),
                };
                let publications_count = u32::try_from(publications.len())
                    .unwrap_or_default()
                    .to_string();
                let publications_html = publications
                    .iter()
                    .map(|p| {
                        let isbn = match p.isbn.clone() {
                            Some(i) => i.to_owned(),
                            None => Default::default(),
                        };
                        html! { <td> { format!("{}({})", isbn, p.publication_type) } </td> }
                    })
                    .collect::<Vec<Html>>();
                let long_abstract = self
                    .book
                    .long_abstract
                    .clone()
                    .unwrap_or_default()
                    .to_string();
                let page_count =
                    format!("{} pages", self.book.page_count.clone().unwrap_or_default());
                let languages = match self.book.languages.clone() {
                    Some(p) => p.to_owned(),
                    None => Default::default(),
                };
                let languages_count = u32::try_from(languages.len())
                    .unwrap_or_default()
                    .to_string();
                let languages_html = languages
                    .iter()
                    .map(|l| {
                        html! { <td> { format!("{}({})", l.language_code, l.language_relation) } </td> }
                    })
                    .collect::<Vec<Html>>();
                let work_status = if let WorkStatus::Forthcoming = self.book.work_status {
                    html! {
                        <span class="inline-block text-xs mr-1 md:mr-2 mb-2 px-2 md:px-4 py-1 opacity-90 text-black bg-yellow-400"> { "FORTHCOMING" }  </span>
                    }
                } else {
                    html! {}
                };

                let fundings = match self.book.fundings.clone() {
                    Some(f) => f.to_owned(),
                    None => Default::default(),
                };
                let fundings_count = u32::try_from(fundings.len())
                    .unwrap_or_default()
                    .to_string();
                let fundings_html = fundings
                    .iter()
                    .map(|f| {
                        let ror_link = format!(
                            "{}{}",
                            ROR_DOMAIN,
                            f.grant_number.clone().unwrap_or_default()
                        );
                        html! {
                            <a href={ror_link} > { f.project_name.clone().unwrap_or_default() }</a>
                        }
                    })
                    .collect::<Vec<Html>>();

                let keywords = match self.book.subjects.clone() {
                    Some(s) => s
                        .clone()
                        .into_iter()
                        .filter(|k| SubjectType::Keyword == k.subject_type)
                        .map(|k| k.subject_code)
                        .collect::<Vec<_>>()
                        .join("; "),
                    None => Default::default(),
                };

                let bisac = match self.book.subjects.clone() {
                    Some(s) => s
                        .clone()
                        .into_iter()
                        .filter(|k| SubjectType::Bisac == k.subject_type)
                        .map(|k| k.subject_code)
                        .collect::<Vec<_>>()
                        .join("; "),
                    None => Default::default(),
                };

                let thema = match self.book.subjects.clone() {
                    Some(s) => s
                        .clone()
                        .into_iter()
                        .filter(|k| SubjectType::Thema == k.subject_type)
                        .map(|k| k.subject_code)
                        .collect::<Vec<_>>()
                        .join("; "),
                    None => Default::default(),
                };

                let doi_url = format!("{}{}", DOI_DOMAIN, self.doi);

                let contributions = match self.book.contributions.clone() {
                    Some(c) => c.to_owned(),
                    None => Default::default(),
                };

                let contributors = contributions.iter().map(|c| {
                    let contribution = c.clone();
                    let ordinal = c.contribution_ordinal.clone();
                    let contributor_full_name = c.contributor.full_name.clone();
                    let orcid_html = if let Some(orcid) = contribution.contributor.orcid {
                        html! {
                            <div class="flex flex-row gap-1">
                                <span class="ai ai-orcid text-orcid pt-1 w-5 h-5" aria-hidden="true"></span>
                                <a href= { format!("{}", orcid.clone()) } title = { format!("{}'s ORCID record", contributor_full_name.clone())} > { orcid } </a>
                            </div>
                        }
                    } else { html!{} };
                    let website_html = if let Some(website) = contribution.contributor.website {
                        html!{
                            <div class="flex flex-col md:flex-row gap-1 md:gap-4 mt-2">
                                <div class="flex flex-row gap-1">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon" class="w-5 h-5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997        8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"></path>
                                    </svg>
                                    <a href= { website.clone() } title= { format!("{}'s website",contributor_full_name.clone()) }> { website } </a>
                                </div>
                            </div>
                        }
                    } else { html! {} };
                    html! {
                        <div class = { if ordinal == 1 { "" }  else { "pt-6" }}>
                            <div class="flex flex-row gap-2 pb-2">
                                <h4 class="font-semibold text-gray-900 dark:text-gray-200"> { c.full_name.clone() } </h4>
                            </div>
                            { orcid_html }
                            { website_html }
                            <p class="mt-4 px-4 border-l-4 border-ternary-400 prose dark:prose-invert max-w-full text-justify"> { c.biography.clone().unwrap_or_default() } </p>
                        </div>
                    }
                }).collect::<Vec<Html>>();

                html! {
                    <div class="container py-12">
                        <div class="flex flex-col lg:flex-row gap-10">
                            <div>
                                <div class="w-max ml-auto mr-auto">
                                    <img src={self.book.cover_url.clone()} alt="Book cover placeholder" height="500" width="333"
                                        sizes="75vw" class="w-32 lg:w-48 object-cover " role="presentation" />
                                </div>
                                <div id="export-metadata" class="px-2 py-4 my-2 hidden lg:block">
                                    <div class="py-4 font-semibold text-header">{"Export Metadata"}</div>
                                    <ul>
                                        <li class="py-1"> { "ONIX 3.0" }
                                            <ul class="list-inside pl-5">
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_3.0::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="0"> { "Thoth" }</a>
                                                </li>
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_3.0::project_muse/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="0"> { "Project MUSE" } </a>
                                                </li>
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_3.0::oapen/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="0"> {"OAPEN"} </a>
                                                </li>
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_3.0::jstor/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="0"> {"JSTOR"} </a>
                                                </li>
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_3.0::google_books/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="0"> {"Google Books"} </a>
                                                </li>
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_3.0::overdrive/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="0"> {"OverDrive"} </a>
                                                </li>
                                            </ul>
                                        </li>
                                        <li class="py-1">{ "ONIX 2.1" }
                                            <ul class="list-inside pl-5">
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_2.1::ebsco_host/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="5"> {"EBSCO Host"} </a>
                                                </li>
                                                <li class="py-1">
                                                    <a href = { format!("{}/specifications/onix_2.1::proquest_ebrary/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                        target="_blank" rel="noopener noreferrer" tabindex="5"> {"ProQuest Ebrary"} </a>
                                                </li>
                                            </ul>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/csv::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"CSV"}</a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/json::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"JSON"}</a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/kbart::oclc/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"OCLC KBART"} </a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/bibtex::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"BibTeX"}</a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/doideposit::crossref/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"CrossRef DOI deposit"}</a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/marc21record::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0"> { "MARC 21 Record" } </a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/marc21markup::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"MARC 21 Markup"}</a>
                                        </li>
                                        <li class="py-1">
                                            <a href = { format!("{}/specifications/marc21xml::thoth/work/{}",THOTH_EXPORT_API,self.book_id.clone()) }
                                                target="_blank" rel="noopener noreferrer" tabindex="0">{"MARC 21 XML"}</a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                            <div class="w-full">
                                <div class="pb-0.5 text-header text-lg"> {self.book.publisher()} </div>
                                <h1 class="text-3xl my-2 text-gray-900 dark:text-gray-100"> {self.book.compile_fulltitle()}</h1>
                                <ul class="my-2 bullet-seperated" role="list">
                                    <span class="inline-block"> { contributors_text.clone()} </span>
                                </ul>
                                <div class="flex flex-row gap-2">
                                    { work_status }
                                </div>
                                <nav class="my-4 pc-5 border-2 border-primary-200 dark:border-gray-400 rounded-2xl">
                                    <ul class="flex flex-wrap md:flex-row list-none gap-2 place-content-center">
                                        <li tabindex="0" aria-label="Export Metadata" role="link">
                                            <a class="flex flex-col items-center w-16 md:w-20 lg:w-24 xl:w-32 text-center text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400 dark:hover:text-primary-400 px-4 py-3
                                                border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline" href="#export-metadata">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 7.5h-.75A2.25 2.25 0 0 0 4.5 9.75v7.5a2.25 2.25 0 0 0 2.25 2.25h7.5a2.25 2.25 0 0 0 2.25-2.25v-7.5a2.25 2.25 0 0 0-2.25-2.25h-.75m-6 3.75 3
                                                        3m0 0 3-3m-3 3V1.5m6 9h.75a2.25 2.25 0 0 1 2.25 2.25v7.5a2.25 2.25 0 0 1-2.25 2.25h-7.5a2.25 2.25 0 0 1-2.25-2.25v-.75">
                                                    </path>
                                                </svg>
                                                <span class="hidden md:block">{"Export Metadata"}</span>
                                            </a>
                                        </li>
                                        <li tabindex="0" aria-label="Metadata" role="link">
                                            <a class="flex flex-col items-center w-16 md:w-20 lg:w-24 xl:w-32 text-center text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400
                                                dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline" href="#metadata">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0 1 19.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 0 0 4.5 10.5a7.464 7.464 0 0 1-1.15 3.993m1.
                                                        989 3.559A11.209 11.209 0 0 0 8.25 10.5a3.75 3.75 0 1 1 7.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 0 1-3.6 9.75m6.633-4.596a18.666 18.666 0 0 1-2.485 5.33">
                                                    </path>
                                                </svg>
                                                <span class="hidden md:block">{"Metadata"}</span>
                                            </a>
                                        </li>
                                        <li tabindex="0" aria-label="Locations" role="link">
                                            <a class="flex flex-col items-center w-16 md:w-20 lg:w-24 xl:w-32 text-center text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400 dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline" href="#locations">
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244"></path>
                                                </svg>
                                                <span class="hidden md:block">{"Locations"}</span>
                                            </a>
                                        </li>
                                        <li tabindex="0" aria-label="Contributors" role="link">
                                            <a class="flex flex-col items-center w-16 md:w-20 lg:w-24 xl:w-32 text-center text-xs text-primary-700 dark:text-gray-400 hover:text-primary-400
                                            dark:hover:text-primary-400 px-4 py-3 border-b-2 border-transparent hover:border-primary-600 dark:hover:border-primary-400 cursor-pointer group hover:no-underline"
                                            href="#contributors"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon" class="w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6 18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0 0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Z"></path>
                                                </svg>
                                                <span class="hidden md:block">{"Contributors"}</span>
                                            </a>
                                        </li>
                                    </ul>
                                </nav>

                                <div id="metadata" class="bg-ternary-200 dark:bg-gray-800 py-6 container overflow-x-auto">
                                    <table class="table-fixed border-separate border-spacing-2 -ml-2" aria-label="Metadata fields">
                                        <caption class="text-left font-semibold text-header pl-2 py-4"> { "Metadata" } </caption>
                                        <tbody>
                                            <tr>
                                                <th class="font-semibold align-top w-40 text-left">{"Title"}</th>
                                                <td>{self.book.compile_fulltitle()}</td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Contributor"}</th>
                                                <td>{contributors_text.clone()} </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"DOI"}</th>
                                                <td> <a href = { doi_url.clone() }> { doi_url } </a> </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Landing page"}</th>
                                                <td> <a href = { landing_page.clone() } target="_blank"> { landing_page } </a> </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"License"}</th>
                                                <td> <a href = { license.clone() } target="_blank"> { license } </a>  </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Copyright"}</th>
                                                <td> { copyright_holder } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Publisher"}</th>
                                                <td> { self.book.publisher() } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Publication place"} </th>
                                                <td> { place_of_publication } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Published on"}</th>
                                                <td>{ publication_date }</td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left" rowspan = { publications_count } >{"ISBN"}</th>
                                                { publications_html }
                                            </tr>

                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Long abstract"} </th>
                                                <td> { long_abstract } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Print length"}</th>
                                                <td>{ page_count } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold} align-top text-left" rowspan = { languages_count }>{"Language"}</th>
                                                { languages_html }
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Dimensions"}</th>
                                                // <td class="pr-4">{"127 x 203 mm | 5 x 8 (Paperback)"}</td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"LCCN"}</th>
                                                <td> { lccn } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold} align-top text-left">{"THEMA"}</th>
                                                <td> { thema } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"BISAC"}</th>
                                                <td> { bisac } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left">{"Keywords"}</th>
                                                <td> { keywords } </td>
                                            </tr>
                                            <tr>
                                                <th class="font-semibold align-top text-left" rowspan = { fundings_count }>{"Funding"}</th>
                                                { fundings_html }
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>

                                <div id="locations" class="bg-ternary-200 dark:bg-gray-800 container py-4 my-2">
                                    <table class="table-fixed border-separate border-spacing-4 lg:border-spacing-2 -mt-4 -ml-4 lg:-mt-2 lg:-ml-2" aria-label="Locations by platform and publication type">
                                        <caption class="text-left font-semibold text-header pl-2 py-5"> {"Locations"} </caption>

                                    </table>
                                </div>

                                <div class="bg-ternary-200 dark:bg-gray-800 container py-4 my-2" id="contributors">
                                    <div class="py-4 font-semibold text-header"> {"Contributors"} </div>
                                    <div class="grid gap-6 divide-y divide-ternary-400">
                                        { contributors }
                                    </div>
                                </div>

                            </div>
                        </div>
                    </div>
                }
            }
            FetchState::Failed(_, err) => html! {
                { ThothError::from(err).to_string() }
            },
        }
    }
}
