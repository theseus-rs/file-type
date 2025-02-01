use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85413270: FileFormat = FileFormat {
    id: 85_413_270,
    puid: "wikidata/85413270",
    name: "PTGui Project File 11",
    extensions: &["pts"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
