use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26211983: FileFormat = FileFormat {
    id: 26_211_983,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 1.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    signatures: &[],
    related_formats: &[],
};
