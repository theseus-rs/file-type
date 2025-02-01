use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74550562: FileFormat = FileFormat {
    id: 74_550_562,
    puid: "wikidata/74550562",
    name: "SAP2000 DataBase",
    extensions: &["sdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
