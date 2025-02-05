use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126087088: FileFormat = FileFormat {
    id: 126_087_088,
    source_type: SourceType::Wikidata,
    name: "IMF Package Composition Playlist",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
