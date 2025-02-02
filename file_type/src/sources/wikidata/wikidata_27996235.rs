use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27996235: FileFormat = FileFormat {
    id: 27_996_235,
    source_type: SourceType::Wikidata,
    name: "FileMaker Pro Database, version 3",
    extensions: &["fp3"],
    media_types: &["application/x-filemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
