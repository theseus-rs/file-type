use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_79242927: FileFormat = FileFormat {
    id: 79_242_927,
    source_type: SourceType::Wikidata,
    name: "Adobe After Effects Graphics",
    extensions: &["aegraphic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
