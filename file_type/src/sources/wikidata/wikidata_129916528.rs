use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129916528: FileFormat = FileFormat {
    id: 129_916_528,
    source_type: SourceType::Wikidata,
    name: "Janet file format",
    extensions: &["janet"],
    media_types: &["application/x-janet", "text/x-janet"],
    internal_signatures: &[],
    related_formats: &[],
};
