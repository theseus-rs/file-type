use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27203973: FileFormat = FileFormat {
    id: 27_203_973,
    puid: "wikidata/27203973",
    name: "OpenDocument Presentation, version 1.1",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
