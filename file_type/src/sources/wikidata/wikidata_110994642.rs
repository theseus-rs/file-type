use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110994642: FileFormat = FileFormat {
    id: 110_994_642,
    source_type: SourceType::Wikidata,
    name: "SnapShot Project File",
    extensions: &["ssp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
