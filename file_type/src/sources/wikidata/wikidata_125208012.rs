use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125208012: FileFormat = FileFormat {
    id: 125_208_012,
    source_type: SourceType::Wikidata,
    name: "TaskJuggler project",
    extensions: &["tjp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
