use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95994246: FileFormat = FileFormat {
    id: 95_994_246,
    source_type: SourceType::Wikidata,
    name: "Agilent Microarray file format",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
