use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66759482: FileFormat = FileFormat {
    id: 66_759_482,
    source_type: SourceType::Wikidata,
    name: "InfoPath Form file",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
