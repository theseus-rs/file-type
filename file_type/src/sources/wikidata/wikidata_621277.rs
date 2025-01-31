use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_621277: FileFormat = FileFormat {
    id: 621_277,
    puid: "wikidata/621277",
    name: "Apple Lossless",
    extensions: &["caf", "m4a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
