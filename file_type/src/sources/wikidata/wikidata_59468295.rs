use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59468295: FileFormat = FileFormat {
    id: 59_468_295,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Data XPT (Windows)",
    extensions: &["xpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
