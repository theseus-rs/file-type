use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129908549: FileFormat = FileFormat {
    id: 129_908_549,
    puid: "wikidata/129908549",
    name: "JAGS file format",
    extensions: &["jag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
