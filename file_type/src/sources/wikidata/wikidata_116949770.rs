use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116949770: FileFormat = FileFormat {
    id: 116_949_770,
    source_type: SourceType::Wikidata,
    name: "Winfax File",
    extensions: &["fxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
