use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979372: FileFormat = FileFormat {
    id: 27_979_372,
    source_type: SourceType::Wikidata,
    name: "Kate",
    extensions: &["ogx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
