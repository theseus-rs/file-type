use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116646759: FileFormat = FileFormat {
    id: 116_646_759,
    puid: "wikidata/116646759",
    name: "eXcelon Studio project",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
