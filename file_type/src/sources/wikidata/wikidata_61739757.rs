use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61739757: FileFormat = FileFormat {
    id: 61_739_757,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 5",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
