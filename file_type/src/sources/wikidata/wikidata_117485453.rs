use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117485453: FileFormat = FileFormat {
    id: 117_485_453,
    source_type: SourceType::Wikidata,
    name: "MacCaption File 2",
    extensions: &["mcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
