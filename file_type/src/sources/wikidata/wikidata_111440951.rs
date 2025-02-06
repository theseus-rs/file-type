use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440951: FileFormat = FileFormat {
    id: 111_440_951,
    source_type: SourceType::Wikidata,
    name: "BASIC Source Code File",
    extensions: &["bas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
