use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27350005: FileFormat = FileFormat {
    id: 27_350_005,
    source_type: SourceType::Wikidata,
    name: "TOPSAR L-Band Polarimetry Data",
    extensions: &["datgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
