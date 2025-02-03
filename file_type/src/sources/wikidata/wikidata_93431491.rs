use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_93431491: FileFormat = FileFormat {
    id: 93_431_491,
    source_type: SourceType::Wikidata,
    name: "Final Draft Document 8",
    extensions: &["fdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
