use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34311311: FileFormat = FileFormat {
    id: 34_311_311,
    source_type: SourceType::Wikidata,
    name: "Sense8 Neutral File Format, plain text variant",
    extensions: &["nff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
