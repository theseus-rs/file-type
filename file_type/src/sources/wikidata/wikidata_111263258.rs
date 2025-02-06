use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263258: FileFormat = FileFormat {
    id: 111_263_258,
    source_type: SourceType::Wikidata,
    name: "Soundcap/SoundEdit instrument",
    extensions: &["dewf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
