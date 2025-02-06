use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73514063: FileFormat = FileFormat {
    id: 73_514_063,
    source_type: SourceType::Wikidata,
    name: "PlayStation Archive",
    extensions: &["psarc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
