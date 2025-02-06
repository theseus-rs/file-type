use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119818987: FileFormat = FileFormat {
    id: 119_818_987,
    source_type: SourceType::Wikidata,
    name: "Final Draft AV Document",
    extensions: &["xav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
