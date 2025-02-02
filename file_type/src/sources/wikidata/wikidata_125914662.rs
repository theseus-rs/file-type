use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125914662: FileFormat = FileFormat {
    id: 125_914_662,
    source_type: SourceType::Wikidata,
    name: "Sandboxels Save File",
    extensions: &["sbxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
