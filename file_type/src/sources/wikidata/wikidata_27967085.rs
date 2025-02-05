use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967085: FileFormat = FileFormat {
    id: 27_967_085,
    source_type: SourceType::Wikidata,
    name: "Jason Page",
    extensions: &["jpn", "jpo", "smp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
