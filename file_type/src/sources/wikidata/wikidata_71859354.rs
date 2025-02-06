use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71859354: FileFormat = FileFormat {
    id: 71_859_354,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 12",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
