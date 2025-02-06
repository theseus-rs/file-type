use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333833: FileFormat = FileFormat {
    id: 111_333_833,
    source_type: SourceType::Wikidata,
    name: "Signed 8-bit PCM data",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
