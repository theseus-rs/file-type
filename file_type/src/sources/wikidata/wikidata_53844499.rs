use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_53844499: FileFormat = FileFormat {
    id: 53_844_499,
    source_type: SourceType::Wikidata,
    name: "BibTeX style file",
    extensions: &["bst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
