use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849583: FileFormat = FileFormat {
    id: 105_849_583,
    puid: "wikidata/105849583",
    name: "16bit DOS COM Crack Soft's cryptor encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
