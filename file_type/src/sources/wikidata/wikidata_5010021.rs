use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5010021: FileFormat = FileFormat {
    id: 5_010_021,
    source_type: SourceType::Wikidata,
    name: "CDX Format",
    extensions: &["cdx"],
    media_types: &["chemical/x-cdx"],
    internal_signatures: &[],
    related_formats: &[],
};
