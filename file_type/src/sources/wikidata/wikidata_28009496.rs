use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009496: FileFormat = FileFormat {
    id: 28_009_496,
    source_type: SourceType::Wikidata,
    name: "Zelda Solarus DX saved game",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
