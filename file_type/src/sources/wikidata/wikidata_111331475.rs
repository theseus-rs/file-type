use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111331475: FileFormat = FileFormat {
    id: 111_331_475,
    source_type: SourceType::Wikidata,
    name: "Mus10 audio file",
    extensions: &["mus10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
