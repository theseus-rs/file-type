use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113578632: FileFormat = FileFormat {
    id: 113_578_632,
    source_type: SourceType::Wikidata,
    name: "MAGIX photo album",
    extensions: &["alb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
