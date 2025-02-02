use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979332: FileFormat = FileFormat {
    id: 27_979_332,
    source_type: SourceType::Wikidata,
    name: "Age of Mythology BAR format",
    extensions: &["bar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
