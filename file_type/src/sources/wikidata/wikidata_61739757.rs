use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61739757: FileFormat = FileFormat {
    id: 61_739_757,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 5",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
