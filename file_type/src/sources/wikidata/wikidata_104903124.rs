use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104903124: FileFormat = FileFormat {
    id: 104_903_124,
    puid: "wikidata/104903124",
    name: "Web Archive Collection Zipped",
    extensions: &["wacz"],
    media_types: &["application/x-wacz"],
    internal_signatures: &[],
    related_formats: &[],
};
