use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167502: FileFormat = FileFormat {
    id: 29_167_502,
    source_type: SourceType::Wikidata,
    name: "Open Web App Manifest",
    extensions: &["webapp"],
    media_types: &["application/x-web-app-manifest+json"],
    internal_signatures: &[],
    related_formats: &[],
};
