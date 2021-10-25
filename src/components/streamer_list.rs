use yew::{
    html,
    services::fetch::FetchTask,
    Callback,
    Component,
    ComponentLink,
    Html,
    Properties,
    ShouldRender,
};

use super::{
    list_pagination::ListPagination,
    streamer_preview::StreamerPreview,
};
use crate::{
    error::Error,
    services::Streamers,
    types::StreamerListInfo,
};

/// List of articles component
pub struct StreamerList {
    streamers: Streamers,
    article_list: Option<StreamerListInfo>,
    response: Callback<Result<StreamerListInfo, Error>>,
    task: Option<FetchTask>,
    current_page: u32,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub filter: StreamerListFilter,
}

pub enum Msg {
    Response(Result<StreamerListInfo, Error>),
    PaginationChanged(u32),
}

/// Filters for article list
#[derive(Clone, Debug)]
pub enum StreamerListFilter {
    All,
    ByElo(String),
    ByCategory(String),
    ByLanguage(String),
}

impl Component for ArticleList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties,
              link: ComponentLink<Self>)
              -> Self {
        StreamerList { streamers: Streamers::new(),
                       streamer_list: None,
                       response: link.callback(Msg::Response),
                       task: None,
                       current_page: 0,
                       props,
                       link }
    }

    fn rendered(&mut self,
                first_render: bool) {
        if first_render {
            self.request();
        }
    }

    fn update(&mut self,
              msg: Self::Message)
              -> ShouldRender {
        match msg {
            Msg::Response(Ok(article_list)) => {
                self.streamer_list = Some(streamer_list);
                self.task = None;
            },
            Msg::Response(Err(_)) => {
                self.task = None;
            },
            Msg::PaginationChanged(current_page) => {
                self.current_page = current_page;
                self.request();
            },
        }
        true
    }

    fn change(&mut self,
              props: Self::Properties)
              -> ShouldRender {
        self.props = props;
        self.current_page = 0;
        self.request();
        false
    }

    fn view(&self) -> Html {
        if let Some(streamer_list) = &self.streamer_list {
            if !streamer_list.streamers.is_empty() {
                let callback = self.link.callback(Msg::PaginationChanged);
                html! {
                    <>
                        {for streamer_list.streamers.iter().map(|streamer| {
                            html! { <StreamerPreview streamer=streamer /> }
                        })}
                        <ListPagination
                            streamers_count=streamer_list.streamers_count
                            current_page=self.current_page
                            callback=callback />
                    </>
                }
            }
            else {
                html! {
                    <div class="streamer-preview">{ "No streamers are here... yet." }</div>
                }
            }
        }
        else {
            html! {
                <div class="streamer-preview">{ "Loading..." }</div>
            }
        }
    }
}

impl StreamerList {
    /// Request apis for filters
    fn request(&mut self) {
        match self.props.filter.clone() {
            StreamerListFilter::All => {
                self.task = Some(self.streamers.all(self.current_page,
                                                    self.response.clone()));
            },
            StreamerListFilter::ByElo(elo) => {
                self.task = Some(self.streamers.by_elo(elo,
                                                       self.current_page,
                                                       self.response.clone()));
            },
            StreamerListFilter::ByCategory(category) => {
                self.task = Some(self.streamers
                                     .by_category(category,
                                                  self.current_page,
                                                  self.response.clone()));
            },
            StreamerListFilter::ByLanguage(language) => {
                self.task = Some(self.streamers
                                     .by_language(language,
                                                  self.current_page,
                                                  self.response.clone()));
            },
            StreamerListFilter::Feed => {
                self.task = Some(self.streamers.feed(self.response.clone()));
            },
        }
    }
}
