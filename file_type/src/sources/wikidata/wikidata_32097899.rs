use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_32097899: FileFormat = FileFormat {
    id: 32_097_899,
    puid: "wikidata/32097899",
    name: "Fallout v2 DAT",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
