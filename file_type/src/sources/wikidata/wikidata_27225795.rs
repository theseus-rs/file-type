use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27225795: FileFormat = FileFormat {
    id: 27_225_795,
    puid: "wikidata/27225795",
    name: "OpenDocument Graphics, version 1.0",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    internal_signatures: &[],
    related_formats: &[],
};
