use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50414080: FileFormat = FileFormat {
    id: 50_414_080,
    source_type: SourceType::Wikidata,
    name: "Lightwright 6 Show File",
    extensions: &["lw6"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
