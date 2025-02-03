use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117485251: FileFormat = FileFormat {
    id: 117_485_251,
    source_type: SourceType::Wikidata,
    name: "Direct Stream Digital Interchange File Format",
    extensions: &["dff"],
    media_types: &["audio/x-dff"],
    internal_signatures: &[],
    related_formats: &[],
};
