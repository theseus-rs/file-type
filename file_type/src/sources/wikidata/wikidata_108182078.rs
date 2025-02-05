use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108182078: FileFormat = FileFormat {
    id: 108_182_078,
    source_type: SourceType::Wikidata,
    name: "Android App Bundle",
    extensions: &["aab"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
