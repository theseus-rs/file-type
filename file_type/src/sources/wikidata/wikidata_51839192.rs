use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51839192: FileFormat = FileFormat {
    id: 51_839_192,
    source_type: SourceType::Wikidata,
    name: "PHP Script Page",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
