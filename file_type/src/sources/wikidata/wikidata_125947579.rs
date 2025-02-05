use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125947579: FileFormat = FileFormat {
    id: 125_947_579,
    source_type: SourceType::Wikidata,
    name: "ICC Profile 2",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    signatures: &[],
    related_formats: &[],
};
