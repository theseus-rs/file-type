use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85513647: FileFormat = FileFormat {
    id: 85_513_647,
    source_type: SourceType::Wikidata,
    name: "Cindex Document, version 4",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
