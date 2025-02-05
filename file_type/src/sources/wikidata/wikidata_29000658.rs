use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000658: FileFormat = FileFormat {
    id: 29_000_658,
    source_type: SourceType::Wikidata,
    name: "PTX",
    extensions: &["ptx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
