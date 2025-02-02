use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59468295: FileFormat = FileFormat {
    id: 59_468_295,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Data XPT (Windows)",
    extensions: &["xpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
