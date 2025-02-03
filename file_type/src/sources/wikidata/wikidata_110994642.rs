use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110994642: FileFormat = FileFormat {
    id: 110_994_642,
    source_type: SourceType::Wikidata,
    name: "SnapShot Project File",
    extensions: &["ssp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
