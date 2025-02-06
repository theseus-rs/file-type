use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71859176: FileFormat = FileFormat {
    id: 71_859_176,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 11",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
