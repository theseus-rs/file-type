use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83276106: FileFormat = FileFormat {
    id: 83_276_106,
    source_type: SourceType::Wikidata,
    name: "Interleaf/Quicksilver file format",
    extensions: &["ildoc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
