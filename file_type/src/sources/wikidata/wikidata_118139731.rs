use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118139731: FileFormat = FileFormat {
    id: 118_139_731,
    puid: "wikidata/118139731",
    name: "Printable Project",
    extensions: &["gwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
