use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71275233: FileFormat = FileFormat {
    id: 71_275_233,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 7",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
