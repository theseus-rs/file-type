use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110098625: FileFormat = FileFormat {
    id: 110_098_625,
    source_type: SourceType::Wikidata,
    name: "Exif Image File Format (Compressed)",
    extensions: &["jpeg", "jpg"],
    media_types: &["image/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
