use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944088: FileFormat = FileFormat {
    id: 29_944_088,
    puid: "wikidata/29944088",
    name: "Sun XML Writer",
    extensions: &["sxw"],
    media_types: &["application/vnd.sun.xml.writer"],
    internal_signatures: &[],
    related_formats: &[],
};
