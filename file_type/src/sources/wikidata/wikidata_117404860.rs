use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117404860: FileFormat = FileFormat {
    id: 117_404_860,
    source_type: SourceType::Wikidata,
    name: "VHDL Output File",
    extensions: &["vho"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
