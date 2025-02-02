use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117448727: FileFormat = FileFormat {
    id: 117_448_727,
    source_type: SourceType::Wikidata,
    name: "Transcriber AG TAG Format",
    extensions: &["tag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
