use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27231654: FileFormat = FileFormat {
    id: 27_231_654,
    source_type: SourceType::Wikidata,
    name: "Tag Image File Format, version 6.0",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[],
};
