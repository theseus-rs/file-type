use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61727559: FileFormat = FileFormat {
    id: 61_727_559,
    source_type: SourceType::Wikidata,
    name: "PrimeOCR file format, version 4",
    extensions: &["pro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
