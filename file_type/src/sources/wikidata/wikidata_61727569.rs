use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61727569: FileFormat = FileFormat {
    id: 61_727_569,
    source_type: SourceType::Wikidata,
    name: "PrimeOCR file format, version 4.2",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
