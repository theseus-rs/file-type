use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851415: FileFormat = FileFormat {
    id: 105_851_415,
    source_type: SourceType::Wikidata,
    name: "T'SoundSystem Source (with rem)",
    extensions: &["tss"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
