use yew::prelude::*;

use crate::components::typography::body_text::BodyText;
use crate::components::typography::header::Header;

#[function_component]
pub fn FormsAndInputsMentalModelStory() -> Html {
    html! {
        <>
            <Header>{"<form />"}</Header>
            <BodyText>
                {"Commonly, whenever a user input is needed we tend to use forms. "}
                {"A form is a group of inputs and a submit button."}
                <br/>
                {"In code we dont use the default browser form functionality, everything is a bit more custom. "}
                {"This is why we use our <Form /> component instead of classic <form />."}
            </BodyText>
            <Header class="mt-2">{"User & Programmer convenience"}</Header>
            <BodyText>
                {"It is super important to fix mistakes as soon as possible. "}
                {"To adhere this rule for users, we have client side validation that shows you the error as soon you type or leave the input. "}
                {"This would create a lot of burden for the programmer as there are a "}
                {"lot of states for input being touched, blurred, errored, valid, disabled and etc..."}
                <br/>
                {"The solution is our FormFieldsMacro ("}
                <a href={"/story/form_fields_macro"}>{"See its story for examples"}</a>
                {")."}
                <br/>
                {"The macro requires to pass certain props (FieldControlProps) to the inputs. "}
                <b>{"Inputs that are meant to be used with the macro always have a prop called \"field_control_props\" "}</b>
                {"If you cannot find an input you need with the prop, then make a new input that wraps the one without the prop."}
            </BodyText>
            <Header class="mt-2">{"Domain Specific Inputs"}</Header>
            <BodyText>
                {"Inputs that start with \"Base\" in their name are always Domain-agnostic. For example - FilteredSelect. "}
                <br/>
                {"When we need something more specific always create a Domain Specific Input instead of using the Base one directly in the form. "}
                {"For example - MccInput is a domain specific version of FilteredSelect"}
            </BodyText>
            <Header class="mt-2">{"Nullable inputs"}</Header>
            <BodyText>
                {"There are 3 use cases for inputs:"}
                <ol>
                    <li>{"Make the input optional"}</li>
                    <li>{"Display the input as required, but allow deleting the value and Validate that value is present"}</li>
                    <li>{"Display the input as required, Don't allow value deletion"}</li>
                </ol>
                {"To make these variations possible we need two versions of an input, for case 1 and 2 a 'XNullableInput' and for case 3 an 'XInput'"}
            </BodyText>
        </>
    }
}
