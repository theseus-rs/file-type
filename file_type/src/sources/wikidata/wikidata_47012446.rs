use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47012446: FileFormat = FileFormat {
    id: 47_012_446,
    source_type: SourceType::Wikidata,
    name: "Microstation CAD Drawing file format",
    extensions: &["dgn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
