use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29944206: FileFormat = FileFormat {
    id: 29_944_206,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Draw template, version 1.0",
    extensions: &["std"],
    media_types: &["application/vnd.sun.xml.draw.template"],
    internal_signatures: &[],
    related_formats: &[],
};
