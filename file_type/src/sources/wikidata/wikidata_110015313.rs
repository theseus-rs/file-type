use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110015313: FileFormat = FileFormat {
    id: 110_015_313,
    source_type: SourceType::Wikidata,
    name: "Autorun Maestro Menu File",
    extensions: &["mnu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
