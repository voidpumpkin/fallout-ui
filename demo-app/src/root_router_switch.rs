use crate::root_page::RootPage;
use crate::root_route::Route;
use crate::story_page::StoryPage;
use fallout_ui::stories::accordion_table_story::AccordionTableStory;
use fallout_ui::stories::base_checkbox_set_story::BaseCheckboxSetStory;
use fallout_ui::stories::base_checkbox_story::BaseCheckboxStory;
use fallout_ui::stories::base_code_area_story::BaseCodeAreaStory;
use fallout_ui::stories::base_file_input_story::BaseFileInputStory;
use fallout_ui::stories::base_filterable_nullable_select_story::BaseFilterableNullableSelectStory;
use fallout_ui::stories::base_input_story::BaseInputStory;
use fallout_ui::stories::base_multi_select_story::BaseMultiSelectStory;
use fallout_ui::stories::base_text_area_story::BaseTextAreaStory;
use fallout_ui::stories::button_story::ButtonStory;
use fallout_ui::stories::callout_story::CalloutStory;
use fallout_ui::stories::divider_story::DividerStory;
use fallout_ui::stories::expandable_list_story::ExpandableListStory;
use fallout_ui::stories::form_fields_macro_story::FormFieldsMacroStory;
use fallout_ui::stories::form_story::FormStory;
use fallout_ui::stories::forms_and_inputs_mental_model_story::FormsAndInputsMentalModelStory;
use fallout_ui::stories::link_story::LinkStory;
use fallout_ui::stories::modal_story::ModalStory;
use fallout_ui::stories::multiple_words_input_story::MultipleWordsInputStory;
use fallout_ui::stories::table_story::TableStory;
use fallout_ui::stories::toasts_story::ToastsStory;
use fallout_ui::stories::typography_story::TypographyStory;
use fallout_ui::stories::vertical_data_list_story::VerticalDataListStory;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn RouterSwitch() -> Html {
    use Route::*;

    let route: Route = use_route().unwrap_or(Root);

    let story = match route {
        Root => return html! {<RootPage />},
        Typography => html! {<TypographyStory />},
        Link => html! {<LinkStory<Route> current_page={Link} />},
        Button => html! {<ButtonStory />},
        Divider => html! {<DividerStory />},
        ExpandableList => html! {<ExpandableListStory />},
        Modal => html! {<ModalStory />},
        FormsAndInputsMentalModel => html! {<FormsAndInputsMentalModelStory />},
        Form => html! {<FormStory />},
        FormFieldsMacro => html! {<FormFieldsMacroStory />},
        BaseInput => html! {<BaseInputStory />},
        BaseFilterableNullableSelect => html! {<BaseFilterableNullableSelectStory />},
        BaseMultiSelect => html! {<BaseMultiSelectStory />},
        BaseFileInput => html! {<BaseFileInputStory />},
        BaseCheckbox => html! { <BaseCheckboxStory /> },
        BaseCheckboxSet => html! { <BaseCheckboxSetStory /> },
        MultipleWordsInput => html! {<MultipleWordsInputStory />},
        BaseTextArea => html! {<BaseTextAreaStory />},
        BaseCodeArea => html! {<BaseCodeAreaStory />},
        Callout => html! {<CalloutStory />},
        VerticalDataList => html! { <VerticalDataListStory /> },
        Table => html! {<TableStory />},
        AccordionTable => html! {<AccordionTableStory />},
        Toasts => html! {<ToastsStory />},
    };

    html! {<StoryPage key={route.to_string()}>{story}</StoryPage>}
}
