use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116969050: FileFormat = FileFormat {
    id: 116_969_050,
    puid: "wikidata/116969050",
    name: "RS Red Storm File",
    extensions: &["rsb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
