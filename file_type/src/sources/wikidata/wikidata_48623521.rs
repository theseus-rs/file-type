use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48623521: FileFormat = FileFormat {
    id: 48_623_521,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 3",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
