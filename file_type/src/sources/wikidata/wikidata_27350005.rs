use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27350005: FileFormat = FileFormat {
    id: 27_350_005,
    puid: "wikidata/27350005",
    name: "TOPSAR L-Band Polarimetry Data",
    extensions: &["datgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
