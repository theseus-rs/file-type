use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85513175: FileFormat = FileFormat {
    id: 85_513_175,
    source_type: SourceType::Wikidata,
    name: "Cindex Document, version 2",
    extensions: &["cdx", "tpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
