use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62571475: FileFormat = FileFormat {
    id: 62_571_475,
    source_type: SourceType::Wikidata,
    name: "Online Description Tool Format",
    extensions: &["odt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
