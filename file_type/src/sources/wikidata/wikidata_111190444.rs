use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111190444: FileFormat = FileFormat {
    id: 111_190_444,
    source_type: SourceType::Wikidata,
    name: "Java Script Command File",
    extensions: &["jsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
