use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8043144: FileFormat = FileFormat {
    id: 8_043_144,
    puid: "wikidata/8043144",
    name: "Xar",
    extensions: &["xar"],
    media_types: &["application/vnd.xara"],
    internal_signatures: &[],
    related_formats: &[],
};
