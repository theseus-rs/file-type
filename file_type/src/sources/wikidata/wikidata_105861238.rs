use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861238: FileFormat = FileFormat {
    id: 105_861_238,
    source_type: SourceType::Wikidata,
    name: "Camtasia Zipped Library",
    extensions: &["libzip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
