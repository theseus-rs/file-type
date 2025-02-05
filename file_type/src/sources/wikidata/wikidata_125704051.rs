use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125704051: FileFormat = FileFormat {
    id: 125_704_051,
    source_type: SourceType::Wikidata,
    name: "StarDraw 2.0 file",
    extensions: &["sgv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
