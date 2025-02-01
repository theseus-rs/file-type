use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65533101: FileFormat = FileFormat {
    id: 65_533_101,
    puid: "wikidata/65533101",
    name: "MealPlan file format",
    extensions: &["pln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
