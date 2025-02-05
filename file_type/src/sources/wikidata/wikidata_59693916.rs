use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59693916: FileFormat = FileFormat {
    id: 59_693_916,
    source_type: SourceType::Wikidata,
    name: "QuadriSpace format",
    extensions: &["qsd", "qsl", "qsm", "qst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
