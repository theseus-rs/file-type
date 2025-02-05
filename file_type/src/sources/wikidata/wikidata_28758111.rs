use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758111: FileFormat = FileFormat {
    id: 28_758_111,
    source_type: SourceType::Wikidata,
    name: "Internet Explorer favorites",
    extensions: &["url"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
