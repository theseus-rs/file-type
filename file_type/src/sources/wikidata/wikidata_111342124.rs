use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342124: FileFormat = FileFormat {
    id: 111_342_124,
    source_type: SourceType::Wikidata,
    name: "Sonic Foundry sample resource file",
    extensions: &["sfr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
