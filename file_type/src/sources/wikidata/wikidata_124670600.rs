use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124670600: FileFormat = FileFormat {
    id: 124_670_600,
    source_type: SourceType::Wikidata,
    name: "PCX, version 0",
    extensions: &["pcx"],
    media_types: &["image/x-pcx"],
    signatures: &[],
    related_formats: &[],
};
