use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125948786: FileFormat = FileFormat {
    id: 125_948_786,
    puid: "wikidata/125948786",
    name: "ICC Profile 4",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile", "application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
