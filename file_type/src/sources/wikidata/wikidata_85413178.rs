use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85413178: FileFormat = FileFormat {
    id: 85_413_178,
    puid: "wikidata/85413178",
    name: "PTGui Project File 10",
    extensions: &["pts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
