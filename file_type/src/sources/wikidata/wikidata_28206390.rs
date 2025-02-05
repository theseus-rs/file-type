use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206390: FileFormat = FileFormat {
    id: 28_206_390,
    source_type: SourceType::Wikidata,
    name: "IRIS CMYK Front End Processor CT",
    extensions: &["ct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
