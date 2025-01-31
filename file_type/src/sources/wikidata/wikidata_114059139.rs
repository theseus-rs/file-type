use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114059139: FileFormat = FileFormat {
    id: 114_059_139,
    puid: "wikidata/114059139",
    name: "OpenDocument Database, version 1.3",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.base"],
    internal_signatures: &[],
    related_formats: &[],
};
