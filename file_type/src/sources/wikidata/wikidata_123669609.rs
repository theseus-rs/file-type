use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123669609: FileFormat = FileFormat {
    id: 123_669_609,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing X8",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
