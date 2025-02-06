use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121544939: FileFormat = FileFormat {
    id: 121_544_939,
    source_type: SourceType::Wikidata,
    name: "At Home 2011 Tax Return File",
    extensions: &["t11"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
