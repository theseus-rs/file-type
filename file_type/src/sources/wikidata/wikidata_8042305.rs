use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_8042305: FileFormat = FileFormat {
    id: 8_042_305,
    source_type: SourceType::Wikidata,
    name: "Extensible Music Format",
    extensions: &["xmf"],
    media_types: &["audio/mobile-xmf", "audio/vnd.nokia.mobile-xmf"],
    signatures: &[],
    related_formats: &[],
};
