use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47490016: FileFormat = FileFormat {
    id: 47_490_016,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 9",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
