use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125019957: FileFormat = FileFormat {
    id: 125_019_957,
    source_type: SourceType::Wikidata,
    name: "GrandView Outline file",
    extensions: &["gv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
