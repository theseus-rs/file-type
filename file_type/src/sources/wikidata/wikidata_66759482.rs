use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66759482: FileFormat = FileFormat {
    id: 66_759_482,
    source_type: SourceType::Wikidata,
    name: "InfoPath Form file",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
