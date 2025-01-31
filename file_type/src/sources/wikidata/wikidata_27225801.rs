use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27225801: FileFormat = FileFormat {
    id: 27_225_801,
    puid: "wikidata/27225801",
    name: "OpenDocument Graphics, version 1.2",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    internal_signatures: &[],
    related_formats: &[],
};
