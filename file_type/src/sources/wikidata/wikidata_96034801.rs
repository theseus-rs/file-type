use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96034801: FileFormat = FileFormat {
    id: 96_034_801,
    source_type: SourceType::Wikidata,
    name: "GXL",
    extensions: &["gxl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
