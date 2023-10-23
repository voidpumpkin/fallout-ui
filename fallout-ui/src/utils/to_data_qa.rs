use convert_case::Case;
use convert_case::Casing;

pub fn to_data_qa(data_qa: impl ToString) -> String {
    data_qa.to_string().to_case(Case::Kebab)
}
