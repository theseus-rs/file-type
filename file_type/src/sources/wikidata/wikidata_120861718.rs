use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120861718: FileFormat = FileFormat {
    id: 120_861_718,
    source_type: SourceType::Wikidata,
    name: "MyDVD Project",
    extensions: &["dvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
