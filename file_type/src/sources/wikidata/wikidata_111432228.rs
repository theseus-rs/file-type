use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111432228: FileFormat = FileFormat {
    id: 111_432_228,
    puid: "wikidata/111432228",
    name: "HTTP File Server Template",
    extensions: &["tpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
