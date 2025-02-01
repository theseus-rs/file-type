use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116145260: FileFormat = FileFormat {
    id: 116_145_260,
    puid: "wikidata/116145260",
    name: "FIT file",
    extensions: &["fit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
