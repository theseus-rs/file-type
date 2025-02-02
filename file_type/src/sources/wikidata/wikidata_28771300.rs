use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771300: FileFormat = FileFormat {
    id: 28_771_300,
    source_type: SourceType::Wikidata,
    name: "Markdeep",
    extensions: &["md.html"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
