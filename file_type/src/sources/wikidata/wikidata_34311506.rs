use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34311506: FileFormat = FileFormat {
    id: 34_311_506,
    source_type: SourceType::Wikidata,
    name: "Sense8 Neutral File Format, binary variant",
    extensions: &["bff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
