use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34306669: FileFormat = FileFormat {
    id: 34_306_669,
    source_type: SourceType::Wikidata,
    name: "Scifer archive XML header",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
