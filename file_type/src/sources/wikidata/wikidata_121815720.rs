use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121815720: FileFormat = FileFormat {
    id: 121_815_720,
    source_type: SourceType::Wikidata,
    name: "HMM Packfile",
    extensions: &["pak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
