use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113663059: FileFormat = FileFormat {
    id: 113_663_059,
    source_type: SourceType::Wikidata,
    name: "Coordinate 3D",
    extensions: &["c3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
