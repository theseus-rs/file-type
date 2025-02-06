use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863113: FileFormat = FileFormat {
    id: 105_863_113,
    source_type: SourceType::Wikidata,
    name: "mzXML",
    extensions: &["mzXML"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
