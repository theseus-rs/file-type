use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47529212: FileFormat = FileFormat {
    id: 47_529_212,
    puid: "wikidata/47529212",
    name: "VLW font file",
    extensions: &["vlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
