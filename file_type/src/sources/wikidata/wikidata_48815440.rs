use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48815440: FileFormat = FileFormat {
    id: 48_815_440,
    source_type: SourceType::Wikidata,
    name: "Framework Database, version 3",
    extensions: &["fw", "fw3"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
