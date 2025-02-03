use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118611163: FileFormat = FileFormat {
    id: 118_611_163,
    source_type: SourceType::Wikidata,
    name: "Resource Template File",
    extensions: &["rct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
