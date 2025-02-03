use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125523900: FileFormat = FileFormat {
    id: 125_523_900,
    source_type: SourceType::Wikidata,
    name: "Olympus RAW Detail Info file",
    extensions: &["ori"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
