use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122412029: FileFormat = FileFormat {
    id: 122_412_029,
    source_type: SourceType::Wikidata,
    name: "FileMaker Runtime File",
    extensions: &["syo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
