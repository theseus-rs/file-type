use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355087: FileFormat = FileFormat {
    id: 111_355_087,
    source_type: SourceType::Wikidata,
    name: "G.711 mu-law US telephony format",
    extensions: &["ulw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
