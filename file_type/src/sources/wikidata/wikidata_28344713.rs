use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344713: FileFormat = FileFormat {
    id: 28_344_713,
    source_type: SourceType::Wikidata,
    name: "Package File",
    extensions: &["pkg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
