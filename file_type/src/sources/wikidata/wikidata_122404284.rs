use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122404284: FileFormat = FileFormat {
    id: 122_404_284,
    source_type: SourceType::Wikidata,
    name: "Pilot Resource File",
    extensions: &["plr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
