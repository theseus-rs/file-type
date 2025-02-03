use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71829168: FileFormat = FileFormat {
    id: 71_829_168,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 3",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
