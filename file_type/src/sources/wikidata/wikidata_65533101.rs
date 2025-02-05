use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65533101: FileFormat = FileFormat {
    id: 65_533_101,
    source_type: SourceType::Wikidata,
    name: "MealPlan file format",
    extensions: &["pln"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
