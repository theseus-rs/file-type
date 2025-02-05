use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342062: FileFormat = FileFormat {
    id: 111_342_062,
    source_type: SourceType::Wikidata,
    name: "Melody Machine compressed SoundFont",
    extensions: &["sfark"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
