use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110098625: FileFormat = FileFormat {
    id: 110_098_625,
    source_type: SourceType::Wikidata,
    name: "Exif Image File Format (Compressed)",
    extensions: &["jpeg", "jpg"],
    media_types: &["image/jpeg"],
    signatures: &[],
    related_formats: &[],
};
