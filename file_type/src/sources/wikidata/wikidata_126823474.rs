use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126823474: FileFormat = FileFormat {
    id: 126_823_474,
    source_type: SourceType::Wikidata,
    name: "Visual F# Script File",
    extensions: &["fsx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
