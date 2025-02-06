use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28731049: FileFormat = FileFormat {
    id: 28_731_049,
    source_type: SourceType::Wikidata,
    name: "Dyalog APL Transfer File format",
    extensions: &["dxf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
