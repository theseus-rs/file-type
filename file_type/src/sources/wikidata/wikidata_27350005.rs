use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27350005: FileFormat = FileFormat {
    id: 27_350_005,
    source_type: SourceType::Wikidata,
    name: "TOPSAR L-Band Polarimetry Data",
    extensions: &["datgr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
