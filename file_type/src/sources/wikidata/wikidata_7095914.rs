use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7095914: FileFormat = FileFormat {
    id: 7_095_914,
    source_type: SourceType::Wikidata,
    name: "OpenXDF",
    extensions: &["xdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
