use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863260: FileFormat = FileFormat {
    id: 27_863_260,
    puid: "wikidata/27863260",
    name: "Audio Interchange File Format, compressed",
    extensions: &["aifc", "aifc"],
    media_types: &["audio/aiff", "audio/x-aiff"],
    internal_signatures: &[],
    related_formats: &[],
};
