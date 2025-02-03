use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47489957: FileFormat = FileFormat {
    id: 47_489_957,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 4",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
