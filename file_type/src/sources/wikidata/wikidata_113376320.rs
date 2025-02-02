use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113376320: FileFormat = FileFormat {
    id: 113_376_320,
    source_type: SourceType::Wikidata,
    name: "XL-Paint format",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
