use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856153: FileFormat = FileFormat {
    id: 105_856_153,
    puid: "wikidata/105856153",
    name: "Macintosh Disk image (BZlib compressed)",
    extensions: &["dmg"],
    media_types: &["application/x-apple-diskimage"],
    internal_signatures: &[],
    related_formats: &[],
};
