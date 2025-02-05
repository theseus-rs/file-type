use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125692808: FileFormat = FileFormat {
    id: 125_692_808,
    source_type: SourceType::Wikidata,
    name: "Pocket Excel Format",
    extensions: &["pxl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
