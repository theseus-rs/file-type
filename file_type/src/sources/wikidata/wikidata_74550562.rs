use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74550562: FileFormat = FileFormat {
    id: 74_550_562,
    source_type: SourceType::Wikidata,
    name: "SAP2000 DataBase",
    extensions: &["sdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
