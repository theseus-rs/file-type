use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111417314: FileFormat = FileFormat {
    id: 111_417_314,
    puid: "wikidata/111417314",
    name: "Borland Turbo C++ Project File",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
