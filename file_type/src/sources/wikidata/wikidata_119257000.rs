use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119257000: FileFormat = FileFormat {
    id: 119_257_000,
    source_type: SourceType::Wikidata,
    name: "PayCycle Import Data",
    extensions: &["pcif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
