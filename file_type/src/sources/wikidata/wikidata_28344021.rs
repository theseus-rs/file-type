use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344021: FileFormat = FileFormat {
    id: 28_344_021,
    source_type: SourceType::Wikidata,
    name: "Imagine Object File",
    extensions: &["iob", "obj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
