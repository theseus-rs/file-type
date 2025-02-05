use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905095: FileFormat = FileFormat {
    id: 29_905_095,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System multi-dimensional database file",
    extensions: &["sas7bmdb", "sm2", "sm7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
