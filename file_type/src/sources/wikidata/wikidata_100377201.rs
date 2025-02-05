use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100377201: FileFormat = FileFormat {
    id: 100_377_201,
    source_type: SourceType::Wikidata,
    name: "HP TRIM Outlook Saved Message File",
    extensions: &["vmbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
