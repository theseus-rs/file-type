use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009215: FileFormat = FileFormat {
    id: 111_009_215,
    puid: "wikidata/111009215",
    name: "PrintMaster Banner File Format",
    extensions: &["ban"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
