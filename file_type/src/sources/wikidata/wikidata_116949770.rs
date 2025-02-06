use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116949770: FileFormat = FileFormat {
    id: 116_949_770,
    source_type: SourceType::Wikidata,
    name: "Winfax File",
    extensions: &["fxs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
