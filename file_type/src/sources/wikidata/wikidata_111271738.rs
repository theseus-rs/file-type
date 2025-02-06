use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111271738: FileFormat = FileFormat {
    id: 111_271_738,
    source_type: SourceType::Wikidata,
    name: "Delusion/XTracker sample format",
    extensions: &["dsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
