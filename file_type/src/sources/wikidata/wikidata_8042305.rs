use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8042305: FileFormat = FileFormat {
    id: 8_042_305,
    puid: "wikidata/8042305",
    name: "Extensible Music Format",
    extensions: &["xmf", "xmf"],
    media_types: &["audio/mobile-xmf", "audio/vnd.nokia.mobile-xmf"],
    internal_signatures: &[],
    related_formats: &[],
};
