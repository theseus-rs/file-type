use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47490016: FileFormat = FileFormat {
    id: 47_490_016,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 9",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
