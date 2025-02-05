use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117104232: FileFormat = FileFormat {
    id: 117_104_232,
    source_type: SourceType::Wikidata,
    name: "Picture it! Publishing File",
    extensions: &["php"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
