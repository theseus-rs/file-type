use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125949223: FileFormat = FileFormat {
    id: 125_949_223,
    puid: "wikidata/125949223",
    name: "ICC Profile iccMAX",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile", "application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
