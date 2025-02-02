use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131621225: FileFormat = FileFormat {
    id: 131_621_225,
    source_type: SourceType::Wikidata,
    name: "Dyna database file format",
    extensions: &["d3plot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
