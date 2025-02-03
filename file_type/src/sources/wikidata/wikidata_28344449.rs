use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344449: FileFormat = FileFormat {
    id: 28_344_449,
    source_type: SourceType::Wikidata,
    name: "SNSF",
    extensions: &["minisnsf", "snsf", "snsflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
