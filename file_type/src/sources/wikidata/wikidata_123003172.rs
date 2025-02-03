use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123003172: FileFormat = FileFormat {
    id: 123_003_172,
    source_type: SourceType::Wikidata,
    name: "Truevision TGA 1.0",
    extensions: &["icb", "tga", "vda", "vst"],
    media_types: &["image/x-targa", "image/x-tga"],
    internal_signatures: &[],
    related_formats: &[],
};
