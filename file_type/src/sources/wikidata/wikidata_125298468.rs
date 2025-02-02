use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125298468: FileFormat = FileFormat {
    id: 125_298_468,
    source_type: SourceType::Wikidata,
    name: "Scribe",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
