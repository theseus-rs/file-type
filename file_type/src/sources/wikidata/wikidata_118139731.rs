use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118139731: FileFormat = FileFormat {
    id: 118_139_731,
    source_type: SourceType::Wikidata,
    name: "Printable Project",
    extensions: &["gwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
