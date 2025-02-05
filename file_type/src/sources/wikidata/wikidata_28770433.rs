use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770433: FileFormat = FileFormat {
    id: 28_770_433,
    source_type: SourceType::Wikidata,
    name: "MARCXML",
    extensions: &["mrcx"],
    media_types: &["application/marc", "application/marcxml+xml"],
    signatures: &[],
    related_formats: &[],
};
