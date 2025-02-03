use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29944575: FileFormat = FileFormat {
    id: 29_944_575,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Calc template, version 1.0",
    extensions: &["stc"],
    media_types: &["application/vnd.sun.xml.calc.template"],
    internal_signatures: &[],
    related_formats: &[],
};
