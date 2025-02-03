use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959801: FileFormat = FileFormat {
    id: 27_959_801,
    source_type: SourceType::Wikidata,
    name: "Ableton Groove File",
    extensions: &["agr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
