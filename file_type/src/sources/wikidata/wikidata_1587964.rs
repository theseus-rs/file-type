use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1587964: FileFormat = FileFormat {
    id: 1_587_964,
    source_type: SourceType::Wikidata,
    name: "Harwell-Boeing file format",
    extensions: &["rua"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
