use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111392536: FileFormat = FileFormat {
    id: 111_392_536,
    source_type: SourceType::Wikidata,
    name: "Bryce 5 File",
    extensions: &["br5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
