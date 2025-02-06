use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122075253: FileFormat = FileFormat {
    id: 122_075_253,
    source_type: SourceType::Wikidata,
    name: "Finale Template File",
    extensions: &["ftm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
