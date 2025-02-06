use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116647547: FileFormat = FileFormat {
    id: 116_647_547,
    source_type: SourceType::Wikidata,
    name: "Form file",
    extensions: &["ofm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
