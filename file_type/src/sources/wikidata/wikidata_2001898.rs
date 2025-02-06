use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2001898: FileFormat = FileFormat {
    id: 2_001_898,
    source_type: SourceType::Wikidata,
    name: "Notation Interchange File Format",
    extensions: &["nif"],
    media_types: &["application/vnd.music-niff"],
    signatures: &[],
    related_formats: &[],
};
