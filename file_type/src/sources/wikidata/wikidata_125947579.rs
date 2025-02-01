use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125947579: FileFormat = FileFormat {
    id: 125_947_579,
    puid: "wikidata/125947579",
    name: "ICC Profile 2",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile", "application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
