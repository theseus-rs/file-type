use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858614: FileFormat = FileFormat {
    id: 105_858_614,
    puid: "wikidata/105858614",
    name: "ImageMagick Machine independent File Format bitmap (with rem)",
    extensions: &["mif", "miff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
