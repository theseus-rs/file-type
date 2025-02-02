use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167841: FileFormat = FileFormat {
    id: 29_167_841,
    source_type: SourceType::Wikidata,
    name: "Outerra",
    extensions: &["otx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
