use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123003172: FileFormat = FileFormat {
    id: 123_003_172,
    puid: "wikidata/123003172",
    name: "Truevision TGA 1.0",
    extensions: &["icb", "icb", "tga", "tga", "vda", "vda", "vst", "vst"],
    media_types: &[
        "image/x-targa",
        "image/x-targa",
        "image/x-targa",
        "image/x-targa",
        "image/x-tga",
        "image/x-tga",
        "image/x-tga",
        "image/x-tga",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
