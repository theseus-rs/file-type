use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10394822: FileFormat = FileFormat {
    id: 10_394_822,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 6.3.2",
    extensions: &["zip", "zipx"],
    media_types: &["application/zip"],
    signatures: &[],
    related_formats: &[],
};
