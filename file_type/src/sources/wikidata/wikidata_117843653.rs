use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117843653: FileFormat = FileFormat {
    id: 117_843_653,
    source_type: SourceType::Wikidata,
    name: "IBM GOCA file",
    extensions: &["gca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
