use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966955: FileFormat = FileFormat {
    id: 27_966_955,
    source_type: SourceType::Wikidata,
    name: "USF",
    extensions: &["miniusf", "usf", "usflib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
