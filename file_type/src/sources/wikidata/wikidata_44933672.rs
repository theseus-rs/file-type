use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44933672: FileFormat = FileFormat {
    id: 44_933_672,
    puid: "wikidata/44933672",
    name: "dBASE IV file format",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
