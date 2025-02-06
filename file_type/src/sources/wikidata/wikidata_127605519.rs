use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127605519: FileFormat = FileFormat {
    id: 127_605_519,
    source_type: SourceType::Wikidata,
    name: "Crystal file format",
    extensions: &["cr"],
    media_types: &["text/x-crystal"],
    signatures: &[],
    related_formats: &[],
};
