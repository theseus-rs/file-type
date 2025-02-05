use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863260: FileFormat = FileFormat {
    id: 27_863_260,
    source_type: SourceType::Wikidata,
    name: "Audio Interchange File Format, compressed",
    extensions: &["aifc"],
    media_types: &["audio/aiff", "audio/x-aiff"],
    signatures: &[],
    related_formats: &[],
};
