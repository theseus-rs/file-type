use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34273453: FileFormat = FileFormat {
    id: 34_273_453,
    source_type: SourceType::Wikidata,
    name: "Keynote Zipped",
    extensions: &["key.zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
