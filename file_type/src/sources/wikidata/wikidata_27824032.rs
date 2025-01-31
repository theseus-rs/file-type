use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824032: FileFormat = FileFormat {
    id: 27_824_032,
    puid: "wikidata/27824032",
    name: "ar, Sixth Edition Unix variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
