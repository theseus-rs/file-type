use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121815720: FileFormat = FileFormat {
    id: 121_815_720,
    puid: "wikidata/121815720",
    name: "HMM Packfile",
    extensions: &["pak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
