use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865583: FileFormat = FileFormat {
    id: 105_865_583,
    source_type: SourceType::Wikidata,
    name: "ProPresenter 4/5 presentation",
    extensions: &["pro4", "pro5"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
