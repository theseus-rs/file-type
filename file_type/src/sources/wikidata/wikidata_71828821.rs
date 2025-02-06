use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71828821: FileFormat = FileFormat {
    id: 71_828_821,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 4",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
