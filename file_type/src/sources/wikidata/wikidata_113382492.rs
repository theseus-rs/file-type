use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113382492: FileFormat = FileFormat {
    id: 113_382_492,
    puid: "wikidata/113382492",
    name: "Roxio Easy Media Creator Classic Creator File 8-10",
    extensions: &["rcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
