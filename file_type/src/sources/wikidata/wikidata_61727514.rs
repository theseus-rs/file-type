use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61727514: FileFormat = FileFormat {
    id: 61_727_514,
    puid: "wikidata/61727514",
    name: "PrimeOCR file format, version 3.9",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
