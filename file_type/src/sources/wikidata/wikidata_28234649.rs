use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28234649: FileFormat = FileFormat {
    id: 28_234_649,
    puid: "wikidata/28234649",
    name: "CCITT Group 3",
    extensions: &["g3"],
    media_types: &["image/g3fax"],
    internal_signatures: &[],
    related_formats: &[],
};
