use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114050725: FileFormat = FileFormat {
    id: 114_050_725,
    source_type: SourceType::Wikidata,
    name: "Canon CIF File",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
