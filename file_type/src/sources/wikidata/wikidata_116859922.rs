use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116859922: FileFormat = FileFormat {
    id: 116_859_922,
    source_type: SourceType::Wikidata,
    name: "Test File",
    extensions: &["tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
