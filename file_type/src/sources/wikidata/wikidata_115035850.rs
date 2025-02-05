use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115035850: FileFormat = FileFormat {
    id: 115_035_850,
    source_type: SourceType::Wikidata,
    name: "Calc602 Project File 1.51",
    extensions: &["pc6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
