use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34306669: FileFormat = FileFormat {
    id: 34_306_669,
    source_type: SourceType::Wikidata,
    name: "Scifer archive XML header",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
