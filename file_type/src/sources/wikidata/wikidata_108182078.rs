use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108182078: FileFormat = FileFormat {
    id: 108_182_078,
    source_type: SourceType::Wikidata,
    name: "Android App Bundle",
    extensions: &["aab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
