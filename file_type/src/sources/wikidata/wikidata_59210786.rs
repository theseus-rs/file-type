use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59210786: FileFormat = FileFormat {
    id: 59_210_786,
    source_type: SourceType::Wikidata,
    name: "BIM Collaboration Format",
    extensions: &["bcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
