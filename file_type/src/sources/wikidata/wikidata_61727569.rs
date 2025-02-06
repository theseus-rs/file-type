use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61727569: FileFormat = FileFormat {
    id: 61_727_569,
    source_type: SourceType::Wikidata,
    name: "PrimeOCR file format, version 4.2",
    extensions: &["pro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
