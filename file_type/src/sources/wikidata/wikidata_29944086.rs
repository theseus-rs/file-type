use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944086: FileFormat = FileFormat {
    id: 29_944_086,
    puid: "wikidata/29944086",
    name: "OpenOffice Impress, version 1.0",
    extensions: &["sxi"],
    media_types: &["application/vnd.sun.xml.impress"],
    internal_signatures: &[],
    related_formats: &[],
};
