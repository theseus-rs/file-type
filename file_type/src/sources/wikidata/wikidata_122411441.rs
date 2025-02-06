use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122411441: FileFormat = FileFormat {
    id: 122_411_441,
    source_type: SourceType::Wikidata,
    name: "DWARF Symbolic File",
    extensions: &["dwf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
