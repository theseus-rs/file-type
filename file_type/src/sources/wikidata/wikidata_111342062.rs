use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342062: FileFormat = FileFormat {
    id: 111_342_062,
    source_type: SourceType::Wikidata,
    name: "Melody Machine compressed SoundFont",
    extensions: &["sfark"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
