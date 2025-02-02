use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111392536: FileFormat = FileFormat {
    id: 111_392_536,
    source_type: SourceType::Wikidata,
    name: "Bryce 5 File",
    extensions: &["br5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
