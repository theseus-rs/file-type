use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28921959: FileFormat = FileFormat {
    id: 28_921_959,
    source_type: SourceType::Wikidata,
    name: "Keyhole Markup Language Zipped",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    signatures: &[],
    related_formats: &[],
};
