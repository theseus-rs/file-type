use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125949223: FileFormat = FileFormat {
    id: 125_949_223,
    source_type: SourceType::Wikidata,
    name: "ICC Profile iccMAX",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    signatures: &[],
    related_formats: &[],
};
