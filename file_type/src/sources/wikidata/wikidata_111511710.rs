use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111511710: FileFormat = FileFormat {
    id: 111_511_710,
    source_type: SourceType::Wikidata,
    name: "TGIF File Format",
    extensions: &["obj", "tgif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
