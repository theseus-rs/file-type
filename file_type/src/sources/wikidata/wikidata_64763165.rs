use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64763165: FileFormat = FileFormat {
    id: 64_763_165,
    source_type: SourceType::Wikidata,
    name: "MapPoint Maps file format",
    extensions: &["ptm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
