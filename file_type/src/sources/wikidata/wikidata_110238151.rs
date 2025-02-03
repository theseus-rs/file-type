use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110238151: FileFormat = FileFormat {
    id: 110_238_151,
    source_type: SourceType::Wikidata,
    name: "SunRF",
    extensions: &["rf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
