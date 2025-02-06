use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125255794: FileFormat = FileFormat {
    id: 125_255_794,
    source_type: SourceType::Wikidata,
    name: "CombiTimeTable",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
