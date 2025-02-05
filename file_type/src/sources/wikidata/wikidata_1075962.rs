use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1075962: FileFormat = FileFormat {
    id: 1_075_962,
    source_type: SourceType::Wikidata,
    name: "RealMedia",
    extensions: &["rm", "rv"],
    media_types: &["application/vnd.rn-realmedia"],
    signatures: &[],
    related_formats: &[],
};
