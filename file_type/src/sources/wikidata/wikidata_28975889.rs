use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975889: FileFormat = FileFormat {
    id: 28_975_889,
    source_type: SourceType::Wikidata,
    name: "Potential Control File",
    extensions: &["pot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
