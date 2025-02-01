use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111512376: FileFormat = FileFormat {
    id: 111_512_376,
    puid: "wikidata/111512376",
    name: "ASEG-GDF2- Data definition file",
    extensions: &["dfn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
