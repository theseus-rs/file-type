use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47167455: FileFormat = FileFormat {
    id: 47_167_455,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Database file format",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
