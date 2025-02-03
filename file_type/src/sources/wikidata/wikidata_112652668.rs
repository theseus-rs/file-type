use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652668: FileFormat = FileFormat {
    id: 112_652_668,
    source_type: SourceType::Wikidata,
    name: "Gold Disk Anim format",
    extensions: &["awm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
