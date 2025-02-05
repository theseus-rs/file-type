use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67126629: FileFormat = FileFormat {
    id: 67_126_629,
    source_type: SourceType::Wikidata,
    name: "Print Artist sign file format",
    extensions: &["sgn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
