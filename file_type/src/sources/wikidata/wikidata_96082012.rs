use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96082012: FileFormat = FileFormat {
    id: 96_082_012,
    source_type: SourceType::Wikidata,
    name: "Standard Product Version 3 format",
    extensions: &["sp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
