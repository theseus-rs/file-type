use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61727602: FileFormat = FileFormat {
    id: 61_727_602,
    source_type: SourceType::Wikidata,
    name: "PrimeOCR file format, version 4.3",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
