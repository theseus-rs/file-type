use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975766: FileFormat = FileFormat {
    id: 28_975_766,
    source_type: SourceType::Wikidata,
    name: "DMO format",
    extensions: &["dmo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
