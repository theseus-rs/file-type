use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59693916: FileFormat = FileFormat {
    id: 59_693_916,
    source_type: SourceType::Wikidata,
    name: "QuadriSpace format",
    extensions: &["qsd", "qsl", "qsm", "qst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
