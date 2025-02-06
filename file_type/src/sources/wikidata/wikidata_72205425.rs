use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72205425: FileFormat = FileFormat {
    id: 72_205_425,
    source_type: SourceType::Wikidata,
    name: "Exchange Offline Address Book",
    extensions: &["lzx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
