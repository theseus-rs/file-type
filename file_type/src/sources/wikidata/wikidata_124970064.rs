use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124970064: FileFormat = FileFormat {
    id: 124_970_064,
    source_type: SourceType::Wikidata,
    name: "MIX index file",
    extensions: &["mixindex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
