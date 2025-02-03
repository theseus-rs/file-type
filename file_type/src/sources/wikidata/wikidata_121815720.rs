use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121815720: FileFormat = FileFormat {
    id: 121_815_720,
    source_type: SourceType::Wikidata,
    name: "HMM Packfile",
    extensions: &["pak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
