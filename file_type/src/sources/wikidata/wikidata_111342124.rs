use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342124: FileFormat = FileFormat {
    id: 111_342_124,
    source_type: SourceType::Wikidata,
    name: "Sonic Foundry sample resource file",
    extensions: &["sfr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
