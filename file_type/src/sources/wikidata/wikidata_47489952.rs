use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47489952: FileFormat = FileFormat {
    id: 47_489_952,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 3",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
