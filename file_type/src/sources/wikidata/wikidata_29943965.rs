use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29943965: FileFormat = FileFormat {
    id: 29_943_965,
    source_type: SourceType::Wikidata,
    name: "Statistical Package for the Social Sciences portable data format",
    extensions: &["por"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
