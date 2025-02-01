use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341669: FileFormat = FileFormat {
    id: 111_341_669,
    puid: "wikidata/111341669",
    name: "Creative Labs FM instrument",
    extensions: &["sbi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
