use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47166177: FileFormat = FileFormat {
    id: 47_166_177,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Drawing file format",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
