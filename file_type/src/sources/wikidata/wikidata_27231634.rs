use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27231634: FileFormat = FileFormat {
    id: 27_231_634,
    source_type: SourceType::Wikidata,
    name: "Tag Image File Format, version 4.0",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[],
};
