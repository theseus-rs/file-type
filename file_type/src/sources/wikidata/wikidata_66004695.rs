use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66004695: FileFormat = FileFormat {
    id: 66_004_695,
    source_type: SourceType::Wikidata,
    name: "ImageStyler File",
    extensions: &["ist"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
