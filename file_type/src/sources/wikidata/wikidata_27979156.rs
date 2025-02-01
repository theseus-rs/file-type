use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979156: FileFormat = FileFormat {
    id: 27_979_156,
    puid: "wikidata/27979156",
    name: "ASCII art",
    extensions: &["asc", "ascii", "txt"],
    media_types: &[
        "text/vnd.ascii-art",
        "text/vnd.ascii-art",
        "text/vnd.ascii-art",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
