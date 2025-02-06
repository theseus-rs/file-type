use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127785772: FileFormat = FileFormat {
    id: 127_785_772,
    source_type: SourceType::Wikidata,
    name: "MEL script",
    extensions: &["mel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
