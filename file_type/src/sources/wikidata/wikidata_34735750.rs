use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34735750: FileFormat = FileFormat {
    id: 34_735_750,
    source_type: SourceType::Wikidata,
    name: "SimTower saved game",
    extensions: &["tdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
