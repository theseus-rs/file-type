use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100425576: FileFormat = FileFormat {
    id: 100_425_576,
    source_type: SourceType::Wikidata,
    name: "Minitab Worksheet, version 12",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
