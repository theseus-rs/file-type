use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111995946: FileFormat = FileFormat {
    id: 111_995_946,
    source_type: SourceType::Wikidata,
    name: "Microsoft Paint File",
    extensions: &["msp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
