use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47233167: FileFormat = FileFormat {
    id: 47_233_167,
    puid: "wikidata/47233167",
    name: "LDR",
    extensions: &["dat", "ldr"],
    media_types: &["application/x-ldraw", "application/x-ldraw"],
    internal_signatures: &[],
    related_formats: &[],
};
