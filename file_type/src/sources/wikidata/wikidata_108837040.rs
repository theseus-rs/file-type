use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837040: FileFormat = FileFormat {
    id: 108_837_040,
    source_type: SourceType::Wikidata,
    name: "Nero DVD-Video Compilation File",
    extensions: &["nrd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
