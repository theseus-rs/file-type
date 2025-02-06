use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129922769: FileFormat = FileFormat {
    id: 129_922_769,
    source_type: SourceType::Wikidata,
    name: "Jasmin file format",
    extensions: &["j"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
