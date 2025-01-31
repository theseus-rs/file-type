use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2816480: FileFormat = FileFormat {
    id: 2_816_480,
    puid: "wikidata/2816480",
    name: "3DXML",
    extensions: &["3dxml"],
    media_types: &["application/x-3dxmlplugin"],
    internal_signatures: &[],
    related_formats: &[],
};
