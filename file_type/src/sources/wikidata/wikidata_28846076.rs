use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28846076: FileFormat = FileFormat {
    id: 28_846_076,
    source_type: SourceType::Wikidata,
    name: "Classification Results File Format",
    extensions: &["clr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
