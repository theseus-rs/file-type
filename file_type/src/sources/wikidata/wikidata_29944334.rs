use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944334: FileFormat = FileFormat {
    id: 29_944_334,
    puid: "wikidata/29944334",
    name: "OpenOffice Impress template, version 1.0",
    extensions: &["sti"],
    media_types: &["application/vnd.sun.xml.impress.template"],
    internal_signatures: &[],
    related_formats: &[],
};
