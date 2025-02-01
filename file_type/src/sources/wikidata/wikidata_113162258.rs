use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113162258: FileFormat = FileFormat {
    id: 113_162_258,
    puid: "wikidata/113162258",
    name: "MyMailManager File",
    extensions: &["mml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
