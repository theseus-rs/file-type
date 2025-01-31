use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125172737: FileFormat = FileFormat {
    id: 125_172_737,
    puid: "wikidata/125172737",
    name: "MyNotex file",
    extensions: &["mnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
