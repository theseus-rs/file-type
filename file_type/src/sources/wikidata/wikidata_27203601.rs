use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27203601: FileFormat = FileFormat {
    id: 27_203_601,
    puid: "wikidata/27203601",
    name: "OpenDocument Text, version 1.2",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    internal_signatures: &[],
    related_formats: &[],
};
