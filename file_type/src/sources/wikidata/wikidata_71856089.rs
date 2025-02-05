use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71856089: FileFormat = FileFormat {
    id: 71_856_089,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 9",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
