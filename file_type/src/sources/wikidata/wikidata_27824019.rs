use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824019: FileFormat = FileFormat {
    id: 27_824_019,
    puid: "wikidata/27824019",
    name: "ar, System V variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
