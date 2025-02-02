use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111355087: FileFormat = FileFormat {
    id: 111_355_087,
    source_type: SourceType::Wikidata,
    name: "G.711 mu-law US telephony format",
    extensions: &["ulw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
