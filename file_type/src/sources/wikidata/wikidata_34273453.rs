use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34273453: FileFormat = FileFormat {
    id: 34_273_453,
    source_type: SourceType::Wikidata,
    name: "Keynote Zipped",
    extensions: &["key.zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
