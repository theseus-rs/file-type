use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206968: FileFormat = FileFormat {
    id: 28_206_968,
    source_type: SourceType::Wikidata,
    name: "Photoshop brush",
    extensions: &["abr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
