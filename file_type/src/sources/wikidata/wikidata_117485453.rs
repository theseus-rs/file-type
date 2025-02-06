use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117485453: FileFormat = FileFormat {
    id: 117_485_453,
    source_type: SourceType::Wikidata,
    name: "MacCaption File 2",
    extensions: &["mcc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
