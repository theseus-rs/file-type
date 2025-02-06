use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71829168: FileFormat = FileFormat {
    id: 71_829_168,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 3",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
