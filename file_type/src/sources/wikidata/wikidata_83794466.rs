use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83794466: FileFormat = FileFormat {
    id: 83_794_466,
    source_type: SourceType::Wikidata,
    name: "FileMaker Pro Database, version 12",
    extensions: &["fmp12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
