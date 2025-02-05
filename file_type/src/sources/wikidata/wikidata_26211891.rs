use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26211891: FileFormat = FileFormat {
    id: 26_211_891,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 6.3.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    signatures: &[],
    related_formats: &[],
};
