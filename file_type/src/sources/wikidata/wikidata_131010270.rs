use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131010270: FileFormat = FileFormat {
    id: 131_010_270,
    puid: "wikidata/131010270",
    name: "Smarty template file",
    extensions: &["tpl"],
    media_types: &["application/x-smarty"],
    internal_signatures: &[],
    related_formats: &[],
};
