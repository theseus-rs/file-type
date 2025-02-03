use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61752032: FileFormat = FileFormat {
    id: 61_752_032,
    source_type: SourceType::Wikidata,
    name: "FileMaker Pro Database, version 7",
    extensions: &["fp7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
