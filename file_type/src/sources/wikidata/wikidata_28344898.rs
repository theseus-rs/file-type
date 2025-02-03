use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344898: FileFormat = FileFormat {
    id: 28_344_898,
    source_type: SourceType::Wikidata,
    name: "Axc",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
