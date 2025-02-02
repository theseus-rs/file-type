use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652217: FileFormat = FileFormat {
    id: 112_652_217,
    source_type: SourceType::Wikidata,
    name: "VIZ Render file format",
    extensions: &["drf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
