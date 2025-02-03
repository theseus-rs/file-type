use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121543029: FileFormat = FileFormat {
    id: 121_543_029,
    source_type: SourceType::Wikidata,
    name: "DeductionPro 2008 Data file",
    extensions: &["d08"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
