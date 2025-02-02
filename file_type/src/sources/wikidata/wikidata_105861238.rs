use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861238: FileFormat = FileFormat {
    id: 105_861_238,
    source_type: SourceType::Wikidata,
    name: "Camtasia Zipped Library",
    extensions: &["libzip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
