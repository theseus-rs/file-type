use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85513340: FileFormat = FileFormat {
    id: 85_513_340,
    source_type: SourceType::Wikidata,
    name: "Cindex Document, version 3",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
