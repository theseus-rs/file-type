use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29944082: FileFormat = FileFormat {
    id: 29_944_082,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Draw, version 1.0",
    extensions: &["sxd"],
    media_types: &["application/vnd.sun.xml.draw"],
    internal_signatures: &[],
    related_formats: &[],
};
