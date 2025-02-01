use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115606990: FileFormat = FileFormat {
    id: 115_606_990,
    puid: "wikidata/115606990",
    name: "VCD Layout File",
    extensions: &["vcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
