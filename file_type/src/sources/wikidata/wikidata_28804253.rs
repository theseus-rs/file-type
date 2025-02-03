use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28804253: FileFormat = FileFormat {
    id: 28_804_253,
    source_type: SourceType::Wikidata,
    name: "Uniform Office Format",
    extensions: &["eof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
