use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74550562: FileFormat = FileFormat {
    id: 74_550_562,
    source_type: SourceType::Wikidata,
    name: "SAP2000 DataBase",
    extensions: &["sdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
