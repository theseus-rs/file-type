use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344898: FileFormat = FileFormat {
    id: 28_344_898,
    source_type: SourceType::Wikidata,
    name: "Axc",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
