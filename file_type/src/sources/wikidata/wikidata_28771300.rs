use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771300: FileFormat = FileFormat {
    id: 28_771_300,
    source_type: SourceType::Wikidata,
    name: "Markdeep",
    extensions: &["md.html"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
