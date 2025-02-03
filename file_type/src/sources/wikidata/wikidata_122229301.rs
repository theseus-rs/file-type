use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122229301: FileFormat = FileFormat {
    id: 122_229_301,
    source_type: SourceType::Wikidata,
    name: "PGP Whole Disk Encryption",
    extensions: &["wde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
