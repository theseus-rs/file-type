use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25305144: FileFormat = FileFormat {
    id: 25_305_144,
    source_type: SourceType::Wikidata,
    name: "ShrinkIt",
    extensions: &["shk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
