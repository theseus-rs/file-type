use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97038139: FileFormat = FileFormat {
    id: 97_038_139,
    source_type: SourceType::Wikidata,
    name: "LEGO Mindstorms EV3 Robot Graphics File",
    extensions: &["rfg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
