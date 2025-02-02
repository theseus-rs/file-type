use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_46441: FileFormat = FileFormat {
    id: 46_441,
    source_type: SourceType::Wikidata,
    name: "Cascading Style Sheets",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
