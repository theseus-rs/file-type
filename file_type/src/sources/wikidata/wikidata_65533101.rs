use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_65533101: FileFormat = FileFormat {
    id: 65_533_101,
    source_type: SourceType::Wikidata,
    name: "MealPlan file format",
    extensions: &["pln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
