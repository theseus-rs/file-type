use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61727559: FileFormat = FileFormat {
    id: 61_727_559,
    puid: "wikidata/61727559",
    name: "PrimeOCR file format, version 4",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
