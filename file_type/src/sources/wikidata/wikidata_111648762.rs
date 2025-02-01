use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111648762: FileFormat = FileFormat {
    id: 111_648_762,
    puid: "wikidata/111648762",
    name: "PrintMaster Scrapbook Page File",
    extensions: &["sbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
