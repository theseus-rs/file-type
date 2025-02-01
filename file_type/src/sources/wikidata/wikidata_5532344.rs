use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5532344: FileFormat = FileFormat {
    id: 5_532_344,
    puid: "wikidata/5532344",
    name: "General feature format",
    extensions: &["gff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
