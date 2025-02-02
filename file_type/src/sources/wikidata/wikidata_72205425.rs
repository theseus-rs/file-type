use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72205425: FileFormat = FileFormat {
    id: 72_205_425,
    source_type: SourceType::Wikidata,
    name: "Exchange Offline Address Book",
    extensions: &["lzx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
