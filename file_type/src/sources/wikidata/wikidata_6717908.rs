use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6717908: FileFormat = FileFormat {
    id: 6_717_908,
    source_type: SourceType::Wikidata,
    name: "MSSTYLES",
    extensions: &["msstyles"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
