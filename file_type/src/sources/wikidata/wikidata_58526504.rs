use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58526504: FileFormat = FileFormat {
    id: 58_526_504,
    source_type: SourceType::Wikidata,
    name: "Enigma Binary File 3+",
    extensions: &["mus"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
