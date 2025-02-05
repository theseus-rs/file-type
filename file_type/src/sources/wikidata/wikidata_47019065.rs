use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47019065: FileFormat = FileFormat {
    id: 47_019_065,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 6",
    extensions: &["pm6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
