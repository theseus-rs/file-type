use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131684360: FileFormat = FileFormat {
    id: 131_684_360,
    source_type: SourceType::Wikidata,
    name: "Sierra IO System file format",
    extensions: &["exdg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
