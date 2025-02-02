use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85513340: FileFormat = FileFormat {
    id: 85_513_340,
    source_type: SourceType::Wikidata,
    name: "Cindex Document, version 3",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
