use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120731961: FileFormat = FileFormat {
    id: 120_731_961,
    source_type: SourceType::Wikidata,
    name: "Amapi Pro file",
    extensions: &["a3p"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
