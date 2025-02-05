use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342080: FileFormat = FileFormat {
    id: 111_342_080,
    source_type: SourceType::Wikidata,
    name: "SoundStage sound data file",
    extensions: &["sfd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
