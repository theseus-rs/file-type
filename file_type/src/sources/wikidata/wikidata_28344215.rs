use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344215: FileFormat = FileFormat {
    id: 28_344_215,
    source_type: SourceType::Wikidata,
    name: "Application Developer Documentation Index",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
