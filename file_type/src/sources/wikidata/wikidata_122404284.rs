use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122404284: FileFormat = FileFormat {
    id: 122_404_284,
    source_type: SourceType::Wikidata,
    name: "Pilot Resource File",
    extensions: &["plr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
