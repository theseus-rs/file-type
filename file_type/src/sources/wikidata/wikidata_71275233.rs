use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71275233: FileFormat = FileFormat {
    id: 71_275_233,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 7",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
