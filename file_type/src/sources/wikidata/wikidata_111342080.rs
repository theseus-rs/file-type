use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342080: FileFormat = FileFormat {
    id: 111_342_080,
    puid: "wikidata/111342080",
    name: "SoundStage sound data file",
    extensions: &["sfd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
