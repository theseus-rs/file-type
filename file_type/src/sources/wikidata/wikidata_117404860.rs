use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117404860: FileFormat = FileFormat {
    id: 117_404_860,
    source_type: SourceType::Wikidata,
    name: "VHDL Output File",
    extensions: &["vho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
