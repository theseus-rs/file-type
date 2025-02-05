use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238151: FileFormat = FileFormat {
    id: 110_238_151,
    source_type: SourceType::Wikidata,
    name: "SunRF",
    extensions: &["rf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
