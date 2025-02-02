use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112669255: FileFormat = FileFormat {
    id: 112_669_255,
    source_type: SourceType::Wikidata,
    name: "JSR-184 format",
    extensions: &["m3g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
