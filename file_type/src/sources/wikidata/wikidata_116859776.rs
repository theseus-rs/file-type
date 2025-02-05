use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116859776: FileFormat = FileFormat {
    id: 116_859_776,
    source_type: SourceType::Wikidata,
    name: "Quicken Payee List",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
