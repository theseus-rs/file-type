use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944575: FileFormat = FileFormat {
    id: 29_944_575,
    puid: "wikidata/29944575",
    name: "OpenOffice Calc template, version 1.0",
    extensions: &["stc"],
    media_types: &["application/vnd.sun.xml.calc.template"],
    internal_signatures: &[],
    related_formats: &[],
};
