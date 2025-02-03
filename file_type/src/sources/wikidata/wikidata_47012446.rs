use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47012446: FileFormat = FileFormat {
    id: 47_012_446,
    source_type: SourceType::Wikidata,
    name: "Microstation CAD Drawing file format",
    extensions: &["dgn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
