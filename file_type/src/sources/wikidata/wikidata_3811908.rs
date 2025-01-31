use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3811908: FileFormat = FileFormat {
    id: 3_811_908,
    puid: "wikidata/3811908",
    name: "karaoke file",
    extensions: &["kar"],
    media_types: &["audio/midi"],
    internal_signatures: &[],
    related_formats: &[],
};
