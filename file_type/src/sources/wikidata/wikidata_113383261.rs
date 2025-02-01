use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113383261: FileFormat = FileFormat {
    id: 113_383_261,
    puid: "wikidata/113383261",
    name: "Roxio Audio Project File 11",
    extensions: &["rox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
