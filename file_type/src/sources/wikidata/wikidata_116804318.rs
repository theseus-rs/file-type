use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116804318: FileFormat = FileFormat {
    id: 116_804_318,
    source_type: SourceType::Wikidata,
    name: "TimeWiz Catalog File",
    extensions: &["twc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
