use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131426238: FileFormat = FileFormat {
    id: 131_426_238,
    source_type: SourceType::Wikidata,
    name: "Whiley file format",
    extensions: &["whiley"],
    media_types: &["text/x-whiley"],
    internal_signatures: &[],
    related_formats: &[],
};
