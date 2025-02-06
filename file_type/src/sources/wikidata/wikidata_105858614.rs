use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858614: FileFormat = FileFormat {
    id: 105_858_614,
    source_type: SourceType::Wikidata,
    name: "ImageMagick Machine independent File Format bitmap (with rem)",
    extensions: &["mif", "miff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
