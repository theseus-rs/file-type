use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71432876: FileFormat = FileFormat {
    id: 71_432_876,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 6",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
