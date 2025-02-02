use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344021: FileFormat = FileFormat {
    id: 28_344_021,
    source_type: SourceType::Wikidata,
    name: "Imagine Object File",
    extensions: &["iob", "obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
