use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967085: FileFormat = FileFormat {
    id: 27_967_085,
    source_type: SourceType::Wikidata,
    name: "Jason Page",
    extensions: &["jpn", "jpo", "smp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
