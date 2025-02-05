use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47462131: FileFormat = FileFormat {
    id: 47_462_131,
    source_type: SourceType::Wikidata,
    name: "Caligari TrueSpace file format (ASCII)",
    extensions: &["cob", "scn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
