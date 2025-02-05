use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661362: FileFormat = FileFormat {
    id: 112_661_362,
    source_type: SourceType::Wikidata,
    name: "Motion Analysis TRC File",
    extensions: &["trc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
