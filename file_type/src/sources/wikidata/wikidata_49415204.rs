use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49415204: FileFormat = FileFormat {
    id: 49_415_204,
    puid: "wikidata/49415204",
    name: "CATIA Project, version 4",
    extensions: &["project"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
