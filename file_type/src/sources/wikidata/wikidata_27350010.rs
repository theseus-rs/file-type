use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27350010: FileFormat = FileFormat {
    id: 27_350_010,
    source_type: SourceType::Wikidata,
    name: "TOPSAR P-Band Polarimetry Data",
    extensions: &["datgr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
