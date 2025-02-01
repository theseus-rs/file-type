use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27203100: FileFormat = FileFormat {
    id: 27_203_100,
    puid: "wikidata/27203100",
    name: "OpenDocument Text, version 1.0",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    internal_signatures: &[],
    related_formats: &[],
};
