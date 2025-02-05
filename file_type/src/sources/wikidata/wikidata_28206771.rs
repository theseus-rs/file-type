use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206771: FileFormat = FileFormat {
    id: 28_206_771,
    source_type: SourceType::Wikidata,
    name: "OS/2 Bitmap Array",
    extensions: &["bga", "bmp", "ico"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
