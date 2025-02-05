use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47489957: FileFormat = FileFormat {
    id: 47_489_957,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 4",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
