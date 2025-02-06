use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123003201: FileFormat = FileFormat {
    id: 123_003_201,
    source_type: SourceType::Wikidata,
    name: "Truevision TGA 2.0",
    extensions: &["icb", "tga", "vda", "vst"],
    media_types: &["image/x-targa", "image/x-tga"],
    signatures: &[],
    related_formats: &[],
};
