use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967518: FileFormat = FileFormat {
    id: 27_967_518,
    source_type: SourceType::Wikidata,
    name: "Matroska Subtitles",
    extensions: &["mks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
