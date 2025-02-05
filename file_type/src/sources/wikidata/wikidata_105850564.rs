use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850564: FileFormat = FileFormat {
    id: 105_850_564,
    source_type: SourceType::Wikidata,
    name: "Camtasia Studio Project (UTF)",
    extensions: &["camproj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
