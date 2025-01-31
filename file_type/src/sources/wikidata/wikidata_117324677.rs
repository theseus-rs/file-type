use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117324677: FileFormat = FileFormat {
    id: 117_324_677,
    puid: "wikidata/117324677",
    name: "User Interface file",
    extensions: &["uir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
