use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51994258: FileFormat = FileFormat {
    id: 51_994_258,
    source_type: SourceType::Wikidata,
    name: "DEC WPS Plus Document",
    extensions: &["wpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
