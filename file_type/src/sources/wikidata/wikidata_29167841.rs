use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167841: FileFormat = FileFormat {
    id: 29_167_841,
    source_type: SourceType::Wikidata,
    name: "Outerra",
    extensions: &["otx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
