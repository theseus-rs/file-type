use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29465145: FileFormat = FileFormat {
    id: 29_465_145,
    source_type: SourceType::Wikidata,
    name: "Valve Material Type",
    extensions: &["vmt"],
    media_types: &["application/vnd.valve.source.material"],
    internal_signatures: &[],
    related_formats: &[],
};
