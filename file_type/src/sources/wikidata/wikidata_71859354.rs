use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71859354: FileFormat = FileFormat {
    id: 71_859_354,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 12",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
