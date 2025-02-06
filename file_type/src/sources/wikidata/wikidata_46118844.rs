use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46118844: FileFormat = FileFormat {
    id: 46_118_844,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance Smartmaster Graphics",
    extensions: &["mas"],
    media_types: &["application/vnd.lotus-freelance"],
    signatures: &[],
    related_formats: &[],
};
