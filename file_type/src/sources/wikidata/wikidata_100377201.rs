use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100377201: FileFormat = FileFormat {
    id: 100_377_201,
    source_type: SourceType::Wikidata,
    name: "HP TRIM Outlook Saved Message File",
    extensions: &["vmbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
