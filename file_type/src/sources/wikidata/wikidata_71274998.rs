use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71274998: FileFormat = FileFormat {
    id: 71_274_998,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 8",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
