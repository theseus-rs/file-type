use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342104: FileFormat = FileFormat {
    id: 111_342_104,
    puid: "wikidata/111342104",
    name: "SoundStage sound info file",
    extensions: &["sfi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
