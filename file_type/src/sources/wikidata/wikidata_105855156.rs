use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855156: FileFormat = FileFormat {
    id: 105_855_156,
    source_type: SourceType::Wikidata,
    name: "Final Draft Script",
    extensions: &["fdx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
