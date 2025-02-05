use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109944419: FileFormat = FileFormat {
    id: 109_944_419,
    source_type: SourceType::Wikidata,
    name: "BriefCase file format",
    extensions: &["brc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
