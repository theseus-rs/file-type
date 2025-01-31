use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113383223: FileFormat = FileFormat {
    id: 113_383_223,
    puid: "wikidata/113383223",
    name: "Roxio Data Project File 11",
    extensions: &["rox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
