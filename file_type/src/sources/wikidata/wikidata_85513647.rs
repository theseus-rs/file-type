use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85513647: FileFormat = FileFormat {
    id: 85_513_647,
    source_type: SourceType::Wikidata,
    name: "Cindex Document, version 4",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
