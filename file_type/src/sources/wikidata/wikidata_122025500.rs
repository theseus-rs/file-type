use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122025500: FileFormat = FileFormat {
    id: 122_025_500,
    source_type: SourceType::Wikidata,
    name: "Scorch web page",
    extensions: &["htm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
