use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118464753: FileFormat = FileFormat {
    id: 118_464_753,
    source_type: SourceType::Wikidata,
    name: "Open Media Framework Interchange 2.0",
    extensions: &["omf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
