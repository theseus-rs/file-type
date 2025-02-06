use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125523900: FileFormat = FileFormat {
    id: 125_523_900,
    source_type: SourceType::Wikidata,
    name: "Olympus RAW Detail Info file",
    extensions: &["ori"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
