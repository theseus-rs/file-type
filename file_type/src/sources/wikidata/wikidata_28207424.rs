use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207424: FileFormat = FileFormat {
    id: 28_207_424,
    source_type: SourceType::Wikidata,
    name: "VEGX",
    extensions: &["egx", "vgx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
