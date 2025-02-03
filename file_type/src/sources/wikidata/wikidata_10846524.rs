use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10846524: FileFormat = FileFormat {
    id: 10_846_524,
    source_type: SourceType::Wikidata,
    name: "Blu-ray Disk Movie",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
