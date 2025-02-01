use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62664835: FileFormat = FileFormat {
    id: 62_664_835,
    puid: "wikidata/62664835",
    name: "Active Server Page",
    extensions: &["asp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
