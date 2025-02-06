use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777718: FileFormat = FileFormat {
    id: 28_777_718,
    source_type: SourceType::Wikidata,
    name: "National Transmission Format",
    extensions: &["ntf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
