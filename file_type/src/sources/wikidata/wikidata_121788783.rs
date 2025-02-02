use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121788783: FileFormat = FileFormat {
    id: 121_788_783,
    source_type: SourceType::Wikidata,
    name: "Yamaha PSR Disk Manager File",
    extensions: &["mng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
