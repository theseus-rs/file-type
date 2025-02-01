use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863192: FileFormat = FileFormat {
    id: 27_863_192,
    puid: "wikidata/27863192",
    name: "Audio Data Interchange Format",
    extensions: &["aac"],
    media_types: &["audio/aac"],
    internal_signatures: &[],
    related_formats: &[],
};
