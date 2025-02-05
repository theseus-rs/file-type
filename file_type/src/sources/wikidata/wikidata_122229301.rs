use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122229301: FileFormat = FileFormat {
    id: 122_229_301,
    source_type: SourceType::Wikidata,
    name: "PGP Whole Disk Encryption",
    extensions: &["wde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
