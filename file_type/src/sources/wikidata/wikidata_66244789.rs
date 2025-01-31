use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66244789: FileFormat = FileFormat {
    id: 66_244_789,
    puid: "wikidata/66244789",
    name: "ScreenCam stand-alone Movies format",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
