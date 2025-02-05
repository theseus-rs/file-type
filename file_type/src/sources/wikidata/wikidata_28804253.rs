use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28804253: FileFormat = FileFormat {
    id: 28_804_253,
    source_type: SourceType::Wikidata,
    name: "Uniform Office Format",
    extensions: &["eof"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
