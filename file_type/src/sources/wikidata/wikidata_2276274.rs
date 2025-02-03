use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2276274: FileFormat = FileFormat {
    id: 2_276_274,
    source_type: SourceType::Wikidata,
    name: "System.map",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
