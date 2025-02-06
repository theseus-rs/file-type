use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120861718: FileFormat = FileFormat {
    id: 120_861_718,
    source_type: SourceType::Wikidata,
    name: "MyDVD Project",
    extensions: &["dvd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
