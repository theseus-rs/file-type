use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34311311: FileFormat = FileFormat {
    id: 34_311_311,
    source_type: SourceType::Wikidata,
    name: "Sense8 Neutral File Format, plain text variant",
    extensions: &["nff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
