use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66685983: FileFormat = FileFormat {
    id: 66_685_983,
    puid: "wikidata/66685983",
    name: "OR3",
    extensions: &["or3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
