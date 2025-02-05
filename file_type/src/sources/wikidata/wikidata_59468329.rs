use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59468329: FileFormat = FileFormat {
    id: 59_468_329,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Data XPT (Unix)",
    extensions: &["xpt"],
    media_types: &["application/x-sas-xport"],
    signatures: &[],
    related_formats: &[],
};
