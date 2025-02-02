use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959943: FileFormat = FileFormat {
    id: 27_959_943,
    source_type: SourceType::Wikidata,
    name: "La Lossless Audio",
    extensions: &["la"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
