use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009492: FileFormat = FileFormat {
    id: 28_009_492,
    source_type: SourceType::Wikidata,
    name: "Warcraft II PUD",
    extensions: &["pud"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
