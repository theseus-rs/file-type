use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118315834: FileFormat = FileFormat {
    id: 118_315_834,
    source_type: SourceType::Wikidata,
    name: "FotoSlate 4.0 Project",
    extensions: &["plp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
