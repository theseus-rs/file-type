use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28758111: FileFormat = FileFormat {
    id: 28_758_111,
    source_type: SourceType::Wikidata,
    name: "Internet Explorer favorites",
    extensions: &["url"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
