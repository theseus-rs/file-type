use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47166067: FileFormat = FileFormat {
    id: 47_166_067,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks file format version 2-3",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
