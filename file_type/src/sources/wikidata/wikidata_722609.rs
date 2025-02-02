use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_722609: FileFormat = FileFormat {
    id: 722_609,
    source_type: SourceType::Wikidata,
    name: "MARC standards",
    extensions: &["marc", "mrc"],
    media_types: &["application/marc"],
    internal_signatures: &[],
    related_formats: &[],
};
