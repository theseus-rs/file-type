use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111588281: FileFormat = FileFormat {
    id: 111_588_281,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Library 2",
    extensions: &["indl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
