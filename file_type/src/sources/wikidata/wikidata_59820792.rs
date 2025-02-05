use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59820792: FileFormat = FileFormat {
    id: 59_820_792,
    source_type: SourceType::Wikidata,
    name: "Corel Presentation Exchange File",
    extensions: &["cmx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
