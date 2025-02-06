use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1143208: FileFormat = FileFormat {
    id: 1_143_208,
    source_type: SourceType::Wikidata,
    name: "Cue sheet",
    extensions: &["cue"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
