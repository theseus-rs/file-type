use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979506: FileFormat = FileFormat {
    id: 27_979_506,
    puid: "wikidata/27979506",
    name: "Photoshop Transfer Function",
    extensions: &["atf"],
    media_types: &["application/x-photoshop"],
    internal_signatures: &[],
    related_formats: &[],
};
