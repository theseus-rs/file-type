use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28728783: FileFormat = FileFormat {
    id: 28_728_783,
    source_type: SourceType::Wikidata,
    name: "VOTable",
    extensions: &["vot", "xml"],
    media_types: &["application/x-votable+xml"],
    signatures: &[],
    related_formats: &[],
};
