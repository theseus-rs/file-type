use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849693: FileFormat = FileFormat {
    id: 105_849_693,
    puid: "wikidata/105849693",
    name: "Motion Capture File Format",
    extensions: &["csm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
