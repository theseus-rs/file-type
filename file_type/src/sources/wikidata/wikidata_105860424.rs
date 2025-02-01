use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860424: FileFormat = FileFormat {
    id: 105_860_424,
    puid: "wikidata/105860424",
    name: "Maxon Resource Creation Tool data",
    extensions: &["rct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
