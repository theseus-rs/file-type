use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012492: FileFormat = FileFormat {
    id: 47_012_492,
    puid: "wikidata/47012492",
    name: "Nota Bene Text file format",
    extensions: &["nb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
