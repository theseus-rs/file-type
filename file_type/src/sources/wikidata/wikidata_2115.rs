use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2115: FileFormat = FileFormat {
    id: 2_115,
    source_type: SourceType::Wikidata,
    name: "XML",
    extensions: &["xml"],
    media_types: &["application/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
