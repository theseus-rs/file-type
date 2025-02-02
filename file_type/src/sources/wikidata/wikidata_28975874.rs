use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975874: FileFormat = FileFormat {
    id: 28_975_874,
    source_type: SourceType::Wikidata,
    name: "OOGL TLIST Group file",
    extensions: &["grp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
