use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263258: FileFormat = FileFormat {
    id: 111_263_258,
    source_type: SourceType::Wikidata,
    name: "Soundcap/SoundEdit instrument",
    extensions: &["dewf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
