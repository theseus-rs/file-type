use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851171: FileFormat = FileFormat {
    id: 105_851_171,
    source_type: SourceType::Wikidata,
    name: "Qt Translation source file",
    extensions: &["ts"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
