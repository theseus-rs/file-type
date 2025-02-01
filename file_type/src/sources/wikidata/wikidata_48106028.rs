use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48106028: FileFormat = FileFormat {
    id: 48_106_028,
    puid: "wikidata/48106028",
    name: "Unisys (Sperry) System Data File",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
