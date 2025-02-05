use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1589482: FileFormat = FileFormat {
    id: 1_589_482,
    source_type: SourceType::Wikidata,
    name: "JT",
    extensions: &["JT"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
