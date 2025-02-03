use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59535034: FileFormat = FileFormat {
    id: 59_535_034,
    source_type: SourceType::Wikidata,
    name: "Stuffit Archive File",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    internal_signatures: &[],
    related_formats: &[],
};
