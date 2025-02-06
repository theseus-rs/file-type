use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863259: FileFormat = FileFormat {
    id: 27_863_259,
    source_type: SourceType::Wikidata,
    name: "Audio Interchange File Format, version 1.3",
    extensions: &["aif", "aiff"],
    media_types: &["audio/aiff", "audio/x-aiff"],
    signatures: &[],
    related_formats: &[],
};
