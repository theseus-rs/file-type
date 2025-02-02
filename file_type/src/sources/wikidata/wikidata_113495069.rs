use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113495069: FileFormat = FileFormat {
    id: 113_495_069,
    source_type: SourceType::Wikidata,
    name: "Calc602 Macro File",
    extensions: &["mc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
