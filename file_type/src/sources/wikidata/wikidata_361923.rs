use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_361923: FileFormat = FileFormat {
    id: 361_923,
    puid: "wikidata/361923",
    name: "Lossless predictive audio compression",
    extensions: &["pac"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
