use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129908549: FileFormat = FileFormat {
    id: 129_908_549,
    source_type: SourceType::Wikidata,
    name: "JAGS file format",
    extensions: &["jag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
