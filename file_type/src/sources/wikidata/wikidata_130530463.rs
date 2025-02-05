use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130530463: FileFormat = FileFormat {
    id: 130_530_463,
    source_type: SourceType::Wikidata,
    name: "Praat script file",
    extensions: &["praat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
