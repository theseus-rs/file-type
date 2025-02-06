use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73515052: FileFormat = FileFormat {
    id: 73_515_052,
    source_type: SourceType::Wikidata,
    name: "Microsoft Printer Definition",
    extensions: &["prd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
