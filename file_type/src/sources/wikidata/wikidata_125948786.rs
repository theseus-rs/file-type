use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125948786: FileFormat = FileFormat {
    id: 125_948_786,
    source_type: SourceType::Wikidata,
    name: "ICC Profile 4",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
