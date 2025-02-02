use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125947579: FileFormat = FileFormat {
    id: 125_947_579,
    source_type: SourceType::Wikidata,
    name: "ICC Profile 2",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
