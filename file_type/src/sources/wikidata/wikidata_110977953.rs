use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110977953: FileFormat = FileFormat {
    id: 110_977_953,
    source_type: SourceType::Wikidata,
    name: "AutoDesk 16-bit Animation File",
    extensions: &["flx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
