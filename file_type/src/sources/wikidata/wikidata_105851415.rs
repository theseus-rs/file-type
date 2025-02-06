use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851415: FileFormat = FileFormat {
    id: 105_851_415,
    source_type: SourceType::Wikidata,
    name: "T'SoundSystem Source (with rem)",
    extensions: &["tss"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
