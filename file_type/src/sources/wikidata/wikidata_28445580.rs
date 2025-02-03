use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445580: FileFormat = FileFormat {
    id: 28_445_580,
    source_type: SourceType::Wikidata,
    name: "Application Developer Documentation Data",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
