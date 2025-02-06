use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967220: FileFormat = FileFormat {
    id: 27_967_220,
    source_type: SourceType::Wikidata,
    name: "SoundFX module",
    extensions: &["sfx", "sfx2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
