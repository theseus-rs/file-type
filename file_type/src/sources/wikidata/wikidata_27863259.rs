use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863259: FileFormat = FileFormat {
    id: 27_863_259,
    puid: "wikidata/27863259",
    name: "Audio Interchange File Format, version 1.3",
    extensions: &["aif", "aif", "aiff", "aiff"],
    media_types: &["audio/aiff", "audio/aiff", "audio/x-aiff", "audio/x-aiff"],
    internal_signatures: &[],
    related_formats: &[],
};
