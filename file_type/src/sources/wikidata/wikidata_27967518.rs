use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967518: FileFormat = FileFormat {
    id: 27_967_518,
    source_type: SourceType::Wikidata,
    name: "Matroska Subtitles",
    extensions: &["mks"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
