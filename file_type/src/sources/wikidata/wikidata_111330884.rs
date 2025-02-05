use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111330884: FileFormat = FileFormat {
    id: 111_330_884,
    source_type: SourceType::Wikidata,
    name: "Musifile MPEG Layer II audio stream",
    extensions: &["mus"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
