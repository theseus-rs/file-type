use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130266833: FileFormat = FileFormat {
    id: 130_266_833,
    source_type: SourceType::Wikidata,
    name: "Macaulay2 file format",
    extensions: &["m2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
