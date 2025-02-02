use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206390: FileFormat = FileFormat {
    id: 28_206_390,
    source_type: SourceType::Wikidata,
    name: "IRIS CMYK Front End Processor CT",
    extensions: &["ct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
