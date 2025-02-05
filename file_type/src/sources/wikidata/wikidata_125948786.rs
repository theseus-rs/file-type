use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125948786: FileFormat = FileFormat {
    id: 125_948_786,
    source_type: SourceType::Wikidata,
    name: "ICC Profile 4",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    signatures: &[],
    related_formats: &[],
};
