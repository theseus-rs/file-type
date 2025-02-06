use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47489943: FileFormat = FileFormat {
    id: 47_489_943,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 2",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
