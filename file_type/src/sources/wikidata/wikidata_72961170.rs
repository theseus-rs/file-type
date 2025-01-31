use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72961170: FileFormat = FileFormat {
    id: 72_961_170,
    puid: "wikidata/72961170",
    name: "Prescription Drug Event format",
    extensions: &["pde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
