use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49415700: FileFormat = FileFormat {
    id: 49_415_700,
    puid: "wikidata/49415700",
    name: "CATIA file format, version 5",
    extensions: &["catmaterial"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
