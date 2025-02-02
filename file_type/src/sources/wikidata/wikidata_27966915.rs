use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966915: FileFormat = FileFormat {
    id: 27_966_915,
    source_type: SourceType::Wikidata,
    name: "NES Sound Format Extended",
    extensions: &["nsfe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
