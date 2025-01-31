use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61727114: FileFormat = FileFormat {
    id: 61_727_114,
    puid: "wikidata/61727114",
    name: "PrimeOCR file format, version 3",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
