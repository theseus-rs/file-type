use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71999678: FileFormat = FileFormat {
    id: 71_999_678,
    source_type: SourceType::Wikidata,
    name: "iTunes CoverFlow data file format",
    extensions: &["itc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
