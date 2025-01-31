use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61727569: FileFormat = FileFormat {
    id: 61_727_569,
    puid: "wikidata/61727569",
    name: "PrimeOCR file format, version 4.2",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
