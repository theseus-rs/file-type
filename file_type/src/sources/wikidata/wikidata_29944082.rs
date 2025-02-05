use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29944082: FileFormat = FileFormat {
    id: 29_944_082,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Draw, version 1.0",
    extensions: &["sxd"],
    media_types: &["application/vnd.sun.xml.draw"],
    signatures: &[],
    related_formats: &[],
};
