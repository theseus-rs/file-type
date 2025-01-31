use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61727504: FileFormat = FileFormat {
    id: 61_727_504,
    puid: "wikidata/61727504",
    name: "PrimeOCR file format, version 3.8",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
