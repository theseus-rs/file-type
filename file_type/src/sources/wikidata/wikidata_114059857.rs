use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114059857: FileFormat = FileFormat {
    id: 114_059_857,
    puid: "wikidata/114059857",
    name: "OpenDocument Presentation, version 1.3",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
