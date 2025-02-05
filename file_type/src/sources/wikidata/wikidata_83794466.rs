use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83794466: FileFormat = FileFormat {
    id: 83_794_466,
    source_type: SourceType::Wikidata,
    name: "FileMaker Pro Database, version 12",
    extensions: &["fmp12"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
