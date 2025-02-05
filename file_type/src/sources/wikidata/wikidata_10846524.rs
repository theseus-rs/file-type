use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10846524: FileFormat = FileFormat {
    id: 10_846_524,
    source_type: SourceType::Wikidata,
    name: "Blu-ray Disk Movie",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
