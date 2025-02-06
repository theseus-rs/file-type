use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47489989: FileFormat = FileFormat {
    id: 47_489_989,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 5.5",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
