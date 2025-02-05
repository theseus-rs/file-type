use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71433176: FileFormat = FileFormat {
    id: 71_433_176,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 5",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
