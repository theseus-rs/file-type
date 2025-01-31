use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111512277: FileFormat = FileFormat {
    id: 111_512_277,
    puid: "wikidata/111512277",
    name: "ASEG-GDF2 Description file",
    extensions: &["des"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
