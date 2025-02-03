use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48801518: FileFormat = FileFormat {
    id: 48_801_518,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 8",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
