use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121463963: FileFormat = FileFormat {
    id: 121_463_963,
    source_type: SourceType::Wikidata,
    name: "Adobe Lightroom Library",
    extensions: &["aglib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
