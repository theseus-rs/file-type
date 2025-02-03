use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85621806: FileFormat = FileFormat {
    id: 85_621_806,
    source_type: SourceType::Wikidata,
    name: "PFS:First Choice Document 3",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
