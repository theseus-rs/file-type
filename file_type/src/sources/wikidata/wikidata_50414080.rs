use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50414080: FileFormat = FileFormat {
    id: 50_414_080,
    source_type: SourceType::Wikidata,
    name: "Lightwright 6 Show File",
    extensions: &["lw6"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
