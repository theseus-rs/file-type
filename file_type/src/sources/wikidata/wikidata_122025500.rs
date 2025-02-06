use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122025500: FileFormat = FileFormat {
    id: 122_025_500,
    source_type: SourceType::Wikidata,
    name: "Scorch web page",
    extensions: &["htm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
