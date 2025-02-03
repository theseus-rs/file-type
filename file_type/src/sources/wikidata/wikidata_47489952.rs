use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47489952: FileFormat = FileFormat {
    id: 47_489_952,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 3",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
