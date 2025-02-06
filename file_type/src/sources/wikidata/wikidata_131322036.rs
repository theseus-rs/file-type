use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131322036: FileFormat = FileFormat {
    id: 131_322_036,
    source_type: SourceType::Wikidata,
    name: "Treetop file format",
    extensions: &["treetop", "tt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
