use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837040: FileFormat = FileFormat {
    id: 108_837_040,
    source_type: SourceType::Wikidata,
    name: "Nero DVD-Video Compilation File",
    extensions: &["nrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
