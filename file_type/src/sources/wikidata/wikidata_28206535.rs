use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206535: FileFormat = FileFormat {
    id: 28_206_535,
    source_type: SourceType::Wikidata,
    name: "Magick Persistent Cache (.mpc file)",
    extensions: &["mpc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
