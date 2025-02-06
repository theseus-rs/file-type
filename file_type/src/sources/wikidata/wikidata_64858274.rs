use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64858274: FileFormat = FileFormat {
    id: 64_858_274,
    source_type: SourceType::Wikidata,
    name: "Corel Presentations Master file format",
    extensions: &["mst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
