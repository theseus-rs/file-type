use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61727514: FileFormat = FileFormat {
    id: 61_727_514,
    source_type: SourceType::Wikidata,
    name: "PrimeOCR file format, version 3.9",
    extensions: &["pro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
