use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960157: FileFormat = FileFormat {
    id: 27_960_157,
    source_type: SourceType::Wikidata,
    name: "Matroska Audio",
    extensions: &["mka"],
    media_types: &["audio/matroska", "audio/x-matroska"],
    signatures: &[],
    related_formats: &[],
};
