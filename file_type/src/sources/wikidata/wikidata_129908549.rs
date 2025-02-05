use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129908549: FileFormat = FileFormat {
    id: 129_908_549,
    source_type: SourceType::Wikidata,
    name: "JAGS file format",
    extensions: &["jag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
