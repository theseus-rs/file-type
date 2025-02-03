use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100297968: FileFormat = FileFormat {
    id: 100_297_968,
    source_type: SourceType::Wikidata,
    name: "Flow Charting file format, version 4",
    extensions: &["gfc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
