use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124670600: FileFormat = FileFormat {
    id: 124_670_600,
    source_type: SourceType::Wikidata,
    name: "PCX, version 0",
    extensions: &["pcx"],
    media_types: &["image/x-pcx"],
    internal_signatures: &[],
    related_formats: &[],
};
