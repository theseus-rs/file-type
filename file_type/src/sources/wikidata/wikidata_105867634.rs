use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867634: FileFormat = FileFormat {
    id: 105_867_634,
    source_type: SourceType::Wikidata,
    name: "Nastran input data",
    extensions: &["nas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
