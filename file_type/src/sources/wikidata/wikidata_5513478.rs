use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5513478: FileFormat = FileFormat {
    id: 5_513_478,
    puid: "wikidata/5513478",
    name: "GIFT",
    extensions: &["gift"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
