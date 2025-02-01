use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61727602: FileFormat = FileFormat {
    id: 61_727_602,
    puid: "wikidata/61727602",
    name: "PrimeOCR file format, version 4.3",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
