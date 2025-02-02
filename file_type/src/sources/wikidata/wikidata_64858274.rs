use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_64858274: FileFormat = FileFormat {
    id: 64_858_274,
    source_type: SourceType::Wikidata,
    name: "Corel Presentations Master file format",
    extensions: &["mst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
