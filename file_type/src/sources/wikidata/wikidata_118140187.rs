use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118140187: FileFormat = FileFormat {
    id: 118_140_187,
    source_type: SourceType::Wikidata,
    name: "Serenade Symbol File",
    extensions: &["sym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
