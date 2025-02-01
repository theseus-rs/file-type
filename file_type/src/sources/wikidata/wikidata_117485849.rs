use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485849: FileFormat = FileFormat {
    id: 117_485_849,
    puid: "wikidata/117485849",
    name: "Audacity Project File 1.x",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
