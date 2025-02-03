use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29944088: FileFormat = FileFormat {
    id: 29_944_088,
    source_type: SourceType::Wikidata,
    name: "Sun XML Writer",
    extensions: &["sxw"],
    media_types: &["application/vnd.sun.xml.writer"],
    internal_signatures: &[],
    related_formats: &[],
};
