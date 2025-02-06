use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47462143: FileFormat = FileFormat {
    id: 47_462_143,
    source_type: SourceType::Wikidata,
    name: "Caligari TrueSpace file format",
    extensions: &["cob", "scn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
