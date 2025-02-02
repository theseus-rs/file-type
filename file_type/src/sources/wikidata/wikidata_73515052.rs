use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73515052: FileFormat = FileFormat {
    id: 73_515_052,
    source_type: SourceType::Wikidata,
    name: "Microsoft Printer Definition",
    extensions: &["prd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
