use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_32097899: FileFormat = FileFormat {
    id: 32_097_899,
    source_type: SourceType::Wikidata,
    name: "Fallout v2 DAT",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
