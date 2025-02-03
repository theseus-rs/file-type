use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445582: FileFormat = FileFormat {
    id: 28_445_582,
    source_type: SourceType::Wikidata,
    name: "AGSC",
    extensions: &["agsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
