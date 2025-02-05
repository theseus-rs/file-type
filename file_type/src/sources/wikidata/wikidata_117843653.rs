use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843653: FileFormat = FileFormat {
    id: 117_843_653,
    source_type: SourceType::Wikidata,
    name: "IBM GOCA file",
    extensions: &["gca"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
