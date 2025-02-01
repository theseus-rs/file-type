use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944551: FileFormat = FileFormat {
    id: 29_944_551,
    puid: "wikidata/29944551",
    name: "OpenOffice Calc, version 1.0",
    extensions: &["sxc"],
    media_types: &["application/vnd.sun.xml.calc"],
    internal_signatures: &[],
    related_formats: &[],
};
