use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115035850: FileFormat = FileFormat {
    id: 115_035_850,
    source_type: SourceType::Wikidata,
    name: "Calc602 Project File 1.51",
    extensions: &["pc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
