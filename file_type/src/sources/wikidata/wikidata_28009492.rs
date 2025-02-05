use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009492: FileFormat = FileFormat {
    id: 28_009_492,
    source_type: SourceType::Wikidata,
    name: "Warcraft II PUD",
    extensions: &["pud"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
