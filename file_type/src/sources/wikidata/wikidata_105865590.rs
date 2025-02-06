use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865590: FileFormat = FileFormat {
    id: 105_865_590,
    source_type: SourceType::Wikidata,
    name: "ProPresenter 6 presentation",
    extensions: &["pro6"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
