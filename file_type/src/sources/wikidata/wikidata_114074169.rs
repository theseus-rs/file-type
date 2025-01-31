use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114074169: FileFormat = FileFormat {
    id: 114_074_169,
    puid: "wikidata/114074169",
    name: "OpenDocument Text, version 1.3",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    internal_signatures: &[],
    related_formats: &[],
};
