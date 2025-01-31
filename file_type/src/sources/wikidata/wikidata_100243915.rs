use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100243915: FileFormat = FileFormat {
    id: 100_243_915,
    puid: "wikidata/100243915",
    name: "Student Writing Center Journal",
    extensions: &["jn", "jnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
