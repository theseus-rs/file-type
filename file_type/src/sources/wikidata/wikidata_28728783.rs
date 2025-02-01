use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28728783: FileFormat = FileFormat {
    id: 28_728_783,
    puid: "wikidata/28728783",
    name: "VOTable",
    extensions: &["vot", "xml"],
    media_types: &["application/x-votable+xml", "application/x-votable+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
