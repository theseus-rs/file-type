use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29944088: FileFormat = FileFormat {
    id: 29_944_088,
    source_type: SourceType::Wikidata,
    name: "Sun XML Writer",
    extensions: &["sxw"],
    media_types: &["application/vnd.sun.xml.writer"],
    signatures: &[],
    related_formats: &[],
};
