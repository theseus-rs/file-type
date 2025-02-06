use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116804318: FileFormat = FileFormat {
    id: 116_804_318,
    source_type: SourceType::Wikidata,
    name: "TimeWiz Catalog File",
    extensions: &["twc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
