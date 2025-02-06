use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850523: FileFormat = FileFormat {
    id: 105_850_523,
    source_type: SourceType::Wikidata,
    name: "Camtasia Studio Project",
    extensions: &["camproj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
