use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122411441: FileFormat = FileFormat {
    id: 122_411_441,
    source_type: SourceType::Wikidata,
    name: "DWARF Symbolic File",
    extensions: &["dwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
