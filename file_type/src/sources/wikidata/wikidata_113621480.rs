use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113621480: FileFormat = FileFormat {
    id: 113_621_480,
    source_type: SourceType::Wikidata,
    name: "LoadRunner Analysis file",
    extensions: &["lra"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
