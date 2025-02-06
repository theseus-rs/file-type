use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131545359: FileFormat = FileFormat {
    id: 131_545_359,
    source_type: SourceType::Wikidata,
    name: "Cloud Optimized Point Cloud file",
    extensions: &["copc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
