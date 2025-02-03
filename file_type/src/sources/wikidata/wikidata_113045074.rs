use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113045074: FileFormat = FileFormat {
    id: 113_045_074,
    source_type: SourceType::Wikidata,
    name: "PTC G-Plug Granite file",
    extensions: &["g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
