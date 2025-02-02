use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28731049: FileFormat = FileFormat {
    id: 28_731_049,
    source_type: SourceType::Wikidata,
    name: "Dyalog APL Transfer File format",
    extensions: &["dxf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
