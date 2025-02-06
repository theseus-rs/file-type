use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206714: FileFormat = FileFormat {
    id: 28_206_714,
    source_type: SourceType::Wikidata,
    name: "Portable Anymap",
    extensions: &["pnm"],
    media_types: &["image/x-portable-anymap"],
    signatures: &[],
    related_formats: &[],
};
