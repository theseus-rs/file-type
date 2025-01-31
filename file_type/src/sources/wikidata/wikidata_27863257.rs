use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863257: FileFormat = FileFormat {
    id: 27_863_257,
    puid: "wikidata/27863257",
    name: "Audio Interchange File Format, version 1.2",
    extensions: &["aif", "aif", "aiff", "aiff"],
    media_types: &["audio/aiff", "audio/aiff", "audio/x-aiff", "audio/x-aiff"],
    internal_signatures: &[],
    related_formats: &[],
};
