use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2276274: FileFormat = FileFormat {
    id: 2_276_274,
    source_type: SourceType::Wikidata,
    name: "System.map",
    extensions: &["map"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
