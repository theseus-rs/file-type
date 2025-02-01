use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111665220: FileFormat = FileFormat {
    id: 111_665_220,
    puid: "wikidata/111665220",
    name: "OpenDocument Text Template",
    extensions: &["ott"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
