use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71433176: FileFormat = FileFormat {
    id: 71_433_176,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 5",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
