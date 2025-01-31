use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65533032: FileFormat = FileFormat {
    id: 65_533_032,
    puid: "wikidata/65533032",
    name: "Menu file format",
    extensions: &["mnu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
