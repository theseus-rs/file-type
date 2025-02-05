use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47516360: FileFormat = FileFormat {
    id: 47_516_360,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog XPT (Unix) v.9.1",
    extensions: &["xpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
