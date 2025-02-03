use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864369: FileFormat = FileFormat {
    id: 105_864_369,
    source_type: SourceType::Wikidata,
    name: "D-Fend Reloaded Profile",
    extensions: &["prof"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
