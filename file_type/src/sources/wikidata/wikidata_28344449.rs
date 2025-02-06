use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344449: FileFormat = FileFormat {
    id: 28_344_449,
    source_type: SourceType::Wikidata,
    name: "SNSF",
    extensions: &["minisnsf", "snsf", "snsflib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
