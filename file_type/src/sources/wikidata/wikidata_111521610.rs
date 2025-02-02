use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111521610: FileFormat = FileFormat {
    id: 111_521_610,
    source_type: SourceType::Wikidata,
    name: "Packed-Ice True Colour Sprites",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
