use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21040924: FileFormat = FileFormat {
    id: 21_040_924,
    source_type: SourceType::Wikidata,
    name: "NoiseTrekker format",
    extensions: &["ntk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
