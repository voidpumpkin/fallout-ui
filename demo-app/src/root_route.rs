use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;
use yew::html::ImplicitClone;
use yew_router::prelude::*;

#[derive(
    Routable, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, strum::Display,
)]
pub enum Route {
    #[at("/")]
    Root,

    #[at("/typography")]
    Typography,

    #[at("/link")]
    Link,

    #[at("/button")]
    Button,

    #[at("/divider")]
    Divider,

    #[at("/expandable_list")]
    ExpandableList,

    #[at("/modal")]
    Modal,

    #[strum(serialize = "Forms And Inputs Mental Model")]
    #[at("/forms_and_inputs_mental_model")]
    FormsAndInputsMentalModel,

    #[at("/form")]
    Form,

    #[at("/form_fields_macro")]
    FormFieldsMacro,

    #[at("/base_input")]
    BaseInput,

    #[at("/base_filterable_nullable_select")]
    BaseFilterableNullableSelect,

    #[at("/base_multi_select")]
    BaseMultiSelect,

    #[at("/base_file_input")]
    BaseFileInput,

    #[at("/base_checkbox")]
    BaseCheckbox,

    #[at("/base_checkbox_set")]
    BaseCheckboxSet,

    #[at("/base_text_area")]
    BaseTextArea,

    #[at("/base_code_area")]
    BaseCodeArea,

    #[at("/callout")]
    Callout,

    #[at("/vertical_data_list")]
    VerticalDataList,

    #[at("/table")]
    Table,

    #[at("/accordion_table")]
    AccordionTable,

    #[at("/multiple_words_input")]
    MultipleWordsInput,

    #[at("/toasts")]
    Toasts,
}

impl ImplicitClone for Route {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum StoryGroup {
    NotGrouped,
    CTA,
    DataDisplay,
    Forms,
}

impl std::fmt::Display for StoryGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use StoryGroup::*;

        match self {
            NotGrouped => write!(f, ""),
            CTA => write!(f, "CTA (Call To Action)"),
            Forms => write!(f, "Forms & User Input"),
            DataDisplay => write!(f, "Data Display"),
        }
    }
}

impl Route {
    pub fn group(&self) -> StoryGroup {
        use Route::*;
        use StoryGroup::*;

        match self {
            Link | Button => CTA,
            FormsAndInputsMentalModel
            | Form
            | FormFieldsMacro
            | BaseInput
            | BaseFilterableNullableSelect
            | BaseMultiSelect
            | BaseFileInput
            | BaseCheckbox
            | BaseCheckboxSet
            | MultipleWordsInput
            | BaseTextArea
            | BaseCodeArea => Forms,
            VerticalDataList | Table | AccordionTable => DataDisplay,
            _ => NotGrouped,
        }
    }
}
