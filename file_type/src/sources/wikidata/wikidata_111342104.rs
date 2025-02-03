use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342104: FileFormat = FileFormat {
    id: 111_342_104,
    source_type: SourceType::Wikidata,
    name: "SoundStage sound info file",
    extensions: &["sfi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
