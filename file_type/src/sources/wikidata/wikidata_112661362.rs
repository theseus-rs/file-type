use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661362: FileFormat = FileFormat {
    id: 112_661_362,
    source_type: SourceType::Wikidata,
    name: "Motion Analysis TRC File",
    extensions: &["trc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
