use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51994258: FileFormat = FileFormat {
    id: 51_994_258,
    source_type: SourceType::Wikidata,
    name: "DEC WPS Plus Document",
    extensions: &["wpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
