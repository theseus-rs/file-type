use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116851698: FileFormat = FileFormat {
    id: 116_851_698,
    source_type: SourceType::Wikidata,
    name: "VersaCheck Data File",
    extensions: &["vdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
