use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119786627: FileFormat = FileFormat {
    id: 119_786_627,
    source_type: SourceType::Wikidata,
    name: "Export v4 File",
    extensions: &["mxp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
