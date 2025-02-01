use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485251: FileFormat = FileFormat {
    id: 117_485_251,
    puid: "wikidata/117485251",
    name: "Direct Stream Digital Interchange File Format",
    extensions: &["dff"],
    media_types: &["audio/x-dff"],
    internal_signatures: &[],
    related_formats: &[],
};
