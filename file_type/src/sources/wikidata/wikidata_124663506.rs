use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124663506: FileFormat = FileFormat {
    id: 124_663_506,
    puid: "wikidata/124663506",
    name: "Transmission X-Ray Microscopy data format",
    extensions: &["txm", "txrm", "xrm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
