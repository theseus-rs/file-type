use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28446959: FileFormat = FileFormat {
    id: 28_446_959,
    puid: "wikidata/28446959",
    name: "Binary Document",
    extensions: &["bdoc"],
    media_types: &["application/vnd.bdoc-1.0"],
    internal_signatures: &[],
    related_formats: &[],
};
