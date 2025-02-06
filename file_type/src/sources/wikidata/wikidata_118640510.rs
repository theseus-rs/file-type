use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118640510: FileFormat = FileFormat {
    id: 118_640_510,
    source_type: SourceType::Wikidata,
    name: "Anime Studio File",
    extensions: &["anme"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
