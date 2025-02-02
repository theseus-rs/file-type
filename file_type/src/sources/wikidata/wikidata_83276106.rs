use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83276106: FileFormat = FileFormat {
    id: 83_276_106,
    source_type: SourceType::Wikidata,
    name: "Interleaf/Quicksilver file format",
    extensions: &["ildoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
