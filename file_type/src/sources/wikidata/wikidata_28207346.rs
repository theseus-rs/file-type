use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207346: FileFormat = FileFormat {
    id: 28_207_346,
    puid: "wikidata/28207346",
    name: "Image Capture Board",
    extensions: &["icb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
