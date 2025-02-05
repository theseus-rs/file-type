use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863192: FileFormat = FileFormat {
    id: 27_863_192,
    source_type: SourceType::Wikidata,
    name: "Audio Data Interchange Format",
    extensions: &["aac"],
    media_types: &["audio/aac"],
    signatures: &[],
    related_formats: &[],
};
