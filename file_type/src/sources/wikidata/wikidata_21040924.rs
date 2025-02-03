use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21040924: FileFormat = FileFormat {
    id: 21_040_924,
    source_type: SourceType::Wikidata,
    name: "NoiseTrekker format",
    extensions: &["ntk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
