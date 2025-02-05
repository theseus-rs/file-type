use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34286712: FileFormat = FileFormat {
    id: 34_286_712,
    source_type: SourceType::Wikidata,
    name: "Processing Development Environment",
    extensions: &["pde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
