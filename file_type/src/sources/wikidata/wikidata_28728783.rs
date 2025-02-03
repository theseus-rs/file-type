use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28728783: FileFormat = FileFormat {
    id: 28_728_783,
    source_type: SourceType::Wikidata,
    name: "VOTable",
    extensions: &["vot", "xml"],
    media_types: &["application/x-votable+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
