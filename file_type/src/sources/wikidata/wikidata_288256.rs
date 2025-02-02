use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_288256: FileFormat = FileFormat {
    id: 288_256,
    source_type: SourceType::Wikidata,
    name: "ACE",
    extensions: &["ace"],
    media_types: &["application/x-ace-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
