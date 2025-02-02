use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967405: FileFormat = FileFormat {
    id: 27_967_405,
    source_type: SourceType::Wikidata,
    name: "Master Tracker module",
    extensions: &["mtr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
