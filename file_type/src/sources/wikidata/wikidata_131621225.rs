use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131621225: FileFormat = FileFormat {
    id: 131_621_225,
    source_type: SourceType::Wikidata,
    name: "Dyna database file format",
    extensions: &["d3plot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
