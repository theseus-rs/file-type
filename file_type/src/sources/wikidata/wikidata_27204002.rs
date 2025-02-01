use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27204002: FileFormat = FileFormat {
    id: 27_204_002,
    puid: "wikidata/27204002",
    name: "OpenDocument Presentation, version 1.2",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
