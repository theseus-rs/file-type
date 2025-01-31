use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123003201: FileFormat = FileFormat {
    id: 123_003_201,
    puid: "wikidata/123003201",
    name: "Truevision TGA 2.0",
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
