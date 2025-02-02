use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205879: FileFormat = FileFormat {
    id: 28_205_879,
    source_type: SourceType::Wikidata,
    name: "CUT",
    extensions: &["cut"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
