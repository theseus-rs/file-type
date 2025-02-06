use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116859922: FileFormat = FileFormat {
    id: 116_859_922,
    source_type: SourceType::Wikidata,
    name: "Test File",
    extensions: &["tst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
