use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47490002: FileFormat = FileFormat {
    id: 47_490_002,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 7",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
