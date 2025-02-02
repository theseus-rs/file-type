use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_64763165: FileFormat = FileFormat {
    id: 64_763_165,
    source_type: SourceType::Wikidata,
    name: "MapPoint Maps file format",
    extensions: &["ptm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
