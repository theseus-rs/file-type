use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122333759: FileFormat = FileFormat {
    id: 122_333_759,
    puid: "wikidata/122333759",
    name: "Logo Design Studio File",
    extensions: &["lds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
