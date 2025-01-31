use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28731049: FileFormat = FileFormat {
    id: 28_731_049,
    puid: "wikidata/28731049",
    name: "Dyalog APL Transfer File format",
    extensions: &["dxf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
