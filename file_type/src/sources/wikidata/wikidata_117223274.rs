use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117223274: FileFormat = FileFormat {
    id: 117_223_274,
    source_type: SourceType::Wikidata,
    name: "LDB File",
    extensions: &["ldb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
