use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21040924: FileFormat = FileFormat {
    id: 21_040_924,
    puid: "wikidata/21040924",
    name: "NoiseTrekker format",
    extensions: &["ntk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
