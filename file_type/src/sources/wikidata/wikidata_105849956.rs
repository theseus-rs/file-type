use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849956: FileFormat = FileFormat {
    id: 105_849_956,
    puid: "wikidata/105849956",
    name: "16bit DOS COM USCC encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
