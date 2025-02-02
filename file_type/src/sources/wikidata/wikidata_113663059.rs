use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113663059: FileFormat = FileFormat {
    id: 113_663_059,
    source_type: SourceType::Wikidata,
    name: "Coordinate 3D",
    extensions: &["c3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
