use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975638: FileFormat = FileFormat {
    id: 28_975_638,
    source_type: SourceType::Wikidata,
    name: "Parasolid",
    extensions: &["x_t"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
