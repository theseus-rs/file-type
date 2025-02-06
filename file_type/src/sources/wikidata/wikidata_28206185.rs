use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206185: FileFormat = FileFormat {
    id: 28_206_185,
    source_type: SourceType::Wikidata,
    name: "GIMP Pattern",
    extensions: &["pat"],
    media_types: &["image/x-gimp-pat"],
    signatures: &[],
    related_formats: &[],
};
