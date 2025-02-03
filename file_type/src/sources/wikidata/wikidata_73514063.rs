use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73514063: FileFormat = FileFormat {
    id: 73_514_063,
    source_type: SourceType::Wikidata,
    name: "PlayStation Archive",
    extensions: &["psarc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
