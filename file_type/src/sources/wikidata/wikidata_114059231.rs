use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114059231: FileFormat = FileFormat {
    id: 114_059_231,
    puid: "wikidata/114059231",
    name: "OpenDocument Graphics, version 1.3",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    internal_signatures: &[],
    related_formats: &[],
};
