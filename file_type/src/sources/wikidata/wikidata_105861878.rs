use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861878: FileFormat = FileFormat {
    id: 105_861_878,
    source_type: SourceType::Wikidata,
    name: "NeXtMidas Macro (with rem)",
    extensions: &["mm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
