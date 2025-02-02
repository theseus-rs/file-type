use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29943965: FileFormat = FileFormat {
    id: 29_943_965,
    source_type: SourceType::Wikidata,
    name: "Statistical Package for the Social Sciences portable data format",
    extensions: &["por"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
