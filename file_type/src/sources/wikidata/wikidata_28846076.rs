use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28846076: FileFormat = FileFormat {
    id: 28_846_076,
    source_type: SourceType::Wikidata,
    name: "Classification Results File Format",
    extensions: &["clr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
