use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62571475: FileFormat = FileFormat {
    id: 62_571_475,
    source_type: SourceType::Wikidata,
    name: "Online Description Tool Format",
    extensions: &["odt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
