use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126087088: FileFormat = FileFormat {
    id: 126_087_088,
    source_type: SourceType::Wikidata,
    name: "IMF Package Composition Playlist",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
