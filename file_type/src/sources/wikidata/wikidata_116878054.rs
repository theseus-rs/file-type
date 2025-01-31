use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116878054: FileFormat = FileFormat {
    id: 116_878_054,
    puid: "wikidata/116878054",
    name: "Address Book Text",
    extensions: &["AB5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
