use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27350010: FileFormat = FileFormat {
    id: 27_350_010,
    source_type: SourceType::Wikidata,
    name: "TOPSAR P-Band Polarimetry Data",
    extensions: &["datgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
