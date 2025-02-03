use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851171: FileFormat = FileFormat {
    id: 105_851_171,
    source_type: SourceType::Wikidata,
    name: "Qt Translation source file",
    extensions: &["ts"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
