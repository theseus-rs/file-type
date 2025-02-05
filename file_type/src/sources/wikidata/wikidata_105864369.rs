use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864369: FileFormat = FileFormat {
    id: 105_864_369,
    source_type: SourceType::Wikidata,
    name: "D-Fend Reloaded Profile",
    extensions: &["prof"],
    media_types: &["text/ini"],
    signatures: &[],
    related_formats: &[],
};
