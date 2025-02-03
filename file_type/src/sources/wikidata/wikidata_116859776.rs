use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116859776: FileFormat = FileFormat {
    id: 116_859_776,
    source_type: SourceType::Wikidata,
    name: "Quicken Payee List",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
