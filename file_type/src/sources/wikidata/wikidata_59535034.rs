use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59535034: FileFormat = FileFormat {
    id: 59_535_034,
    source_type: SourceType::Wikidata,
    name: "Stuffit Archive File",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    signatures: &[],
    related_formats: &[],
};
