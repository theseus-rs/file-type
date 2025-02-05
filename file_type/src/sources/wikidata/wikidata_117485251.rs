use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117485251: FileFormat = FileFormat {
    id: 117_485_251,
    source_type: SourceType::Wikidata,
    name: "Direct Stream Digital Interchange File Format",
    extensions: &["dff"],
    media_types: &["audio/x-dff"],
    signatures: &[],
    related_formats: &[],
};
