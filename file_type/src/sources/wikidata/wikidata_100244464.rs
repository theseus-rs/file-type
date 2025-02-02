use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100244464: FileFormat = FileFormat {
    id: 100_244_464,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Letter",
    extensions: &["lt", "ltt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
