use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122047541: FileFormat = FileFormat {
    id: 122_047_541,
    source_type: SourceType::Wikidata,
    name: "cc:Mail Archive Format",
    extensions: &["cca"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
