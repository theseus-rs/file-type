use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119257000: FileFormat = FileFormat {
    id: 119_257_000,
    puid: "wikidata/119257000",
    name: "PayCycle Import Data",
    extensions: &["pcif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
