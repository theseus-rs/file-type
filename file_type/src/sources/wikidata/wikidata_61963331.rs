use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61963331: FileFormat = FileFormat {
    id: 61_963_331,
    source_type: SourceType::Wikidata,
    name: "pulse EKKO data file",
    extensions: &["dt1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
